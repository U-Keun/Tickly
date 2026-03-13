use super::*;

impl RealtimeService {
    pub(super) fn spawn_connection_loop(
        &self,
        config: RealtimeConfig,
        mut shutdown_rx: mpsc::Receiver<()>,
    ) {
        let state = self.state.clone();
        let reconnect_attempts = self.reconnect_attempts.clone();
        let last_error = self.last_error.clone();
        let is_running = self.is_running.clone();
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            loop {
                // Check if we should stop
                if !is_running.load(Ordering::SeqCst) {
                    break;
                }

                // Try to connect
                *state.write().await = RealtimeConnectionState::Connecting;

                match Self::run_connection(&config, &app_handle, &mut shutdown_rx, &state).await {
                    Ok(()) => {
                        // Clean disconnect, don't reconnect
                        break;
                    }
                    Err(e) => {
                        log::error!("Realtime connection error: {}", e);
                        *last_error.write().await = Some(e.clone());

                        // Emit error event
                        let _ = app_handle.emit(
                            "realtime-event",
                            RealtimeEvent {
                                event_type: RealtimeEventType::Error,
                                message: Some(e),
                            },
                        );

                        // Check reconnect attempts
                        let attempts = reconnect_attempts.fetch_add(1, Ordering::SeqCst) + 1;
                        if attempts >= MAX_RECONNECT_ATTEMPTS {
                            log::error!(
                                "Max reconnect attempts ({}) reached, giving up",
                                MAX_RECONNECT_ATTEMPTS
                            );
                            is_running.store(false, Ordering::SeqCst);
                            *state.write().await = RealtimeConnectionState::Disconnected;
                            break;
                        }

                        // Emit reconnecting event
                        *state.write().await = RealtimeConnectionState::Reconnecting;
                        let _ = app_handle.emit(
                            "realtime-event",
                            RealtimeEvent {
                                event_type: RealtimeEventType::Reconnecting,
                                message: Some(format!(
                                    "Attempt {} of {}",
                                    attempts, MAX_RECONNECT_ATTEMPTS
                                )),
                            },
                        );

                        // Calculate backoff delay
                        let delay = std::cmp::min(
                            INITIAL_RECONNECT_DELAY_MS * 2u64.pow(attempts - 1),
                            MAX_RECONNECT_DELAY_MS,
                        );

                        log::info!("Reconnecting in {}ms (attempt {})", delay, attempts);

                        // Wait before reconnecting, but check for shutdown
                        tokio::select! {
                            _ = sleep(Duration::from_millis(delay)) => {}
                            _ = shutdown_rx.recv() => {
                                log::info!("Shutdown signal received during reconnect wait");
                                break;
                            }
                        }
                    }
                }
            }

            // Final cleanup
            *state.write().await = RealtimeConnectionState::Disconnected;
            is_running.store(false, Ordering::SeqCst);
        });
    }

    async fn run_connection(
        config: &RealtimeConfig,
        app_handle: &AppHandle,
        shutdown_rx: &mut mpsc::Receiver<()>,
        state: &Arc<RwLock<RealtimeConnectionState>>,
    ) -> Result<(), String> {
        let url = config.websocket_url();
        log::info!("Connecting to Supabase Realtime");

        // Connect to WebSocket
        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| format!("WebSocket connection failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        // Subscribe to channel
        let channel_topic = "realtime:public".to_string();
        let join_ref = "1";
        let msg_ref = "1";

        // Subscribe to postgres_changes for todos, categories, completion_logs
        let join_payload = serde_json::json!({
            "config": {
                "broadcast": {
                    "ack": false,
                    "self": false
                },
                "presence": {
                    "key": "",
                    "enabled": false
                },
                "postgres_changes": [
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "todos",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "categories",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "completion_logs",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "tags",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    },
                    {
                        "event": "*",
                        "schema": "public",
                        "table": "todo_tags",
                        "filter": format!("user_id=eq.{}", config.user_id)
                    }
                ],
                "private": false
            }
        });

        let join_msg = PhoenixMessage::new(
            Some(join_ref.to_string()),
            Some(msg_ref.to_string()),
            &channel_topic,
            "phx_join",
            join_payload,
        );

        // Send join message
        write
            .send(Message::Text(join_msg.to_json()))
            .await
            .map_err(|e| format!("Failed to send join message: {}", e))?;

        // Send access_token event immediately after join
        let access_token_msg = PhoenixMessage::new(
            Some(join_ref.to_string()),
            Some("2".to_string()),
            &channel_topic,
            "access_token",
            serde_json::json!({
                "access_token": config.access_token
            }),
        );

        write
            .send(Message::Text(access_token_msg.to_json()))
            .await
            .map_err(|e| format!("Failed to send access_token: {}", e))?;

        // Start heartbeat interval
        let mut heartbeat_interval = interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        let mut msg_counter: u32 = 10;

        loop {
            tokio::select! {
                // Handle incoming messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Err(e) = Self::handle_message(&text, app_handle, state).await {
                                log::warn!("Failed to handle message: {}", e);
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            return Err("Connection closed by server".to_string());
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = write.send(Message::Pong(data)).await;
                        }
                        Some(Err(e)) => {
                            return Err(format!("WebSocket error: {}", e));
                        }
                        None => {
                            return Err("WebSocket stream ended".to_string());
                        }
                        _ => {}
                    }
                }

                // Send heartbeat
                _ = heartbeat_interval.tick() => {
                    msg_counter += 1;
                    let heartbeat = PhoenixMessage::new(
                        None,
                        Some(msg_counter.to_string()),
                        "phoenix",
                        "heartbeat",
                        serde_json::json!({}),
                    );

                    if let Err(e) = write.send(Message::Text(heartbeat.to_json())).await {
                        return Err(format!("Failed to send heartbeat: {}", e));
                    }
                }

                // Handle shutdown signal
                _ = shutdown_rx.recv() => {
                    log::info!("Realtime shutdown signal received");

                    // Send phx_leave
                    let leave_msg = PhoenixMessage::new(
                        Some(join_ref.to_string()),
                        Some("999".to_string()),
                        &channel_topic,
                        "phx_leave",
                        serde_json::json!({}),
                    );
                    let _ = write.send(Message::Text(leave_msg.to_json())).await;
                    let _ = write.send(Message::Close(None)).await;

                    return Ok(());
                }
            }
        }
    }
}
