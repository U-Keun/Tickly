use super::super::*;

impl SupabaseClient {
    fn build_todo_payload(
        todo: &RemoteTodo,
        include_reminder_at: bool,
        include_linked_app: bool,
    ) -> serde_json::Value {
        let mut payload = serde_json::json!({
            "id": todo.id,
            "user_id": todo.user_id,
            "category_id": todo.category_id,
            "text": todo.text,
            "done": todo.done,
            "display_order": todo.display_order,
            "memo": todo.memo,
            "repeat_type": todo.repeat_type,
            "repeat_detail": todo.repeat_detail,
            "next_due_at": todo.next_due_at,
            "last_completed_at": todo.last_completed_at,
            "track_streak": todo.track_streak,
            "created_at": todo.created_at,
            "updated_at": todo.updated_at,
        });

        if include_reminder_at {
            payload["reminder_at"] =
                serde_json::to_value(&todo.reminder_at).unwrap_or(serde_json::Value::Null);
        }

        if include_linked_app {
            payload["linked_app"] =
                serde_json::to_value(&todo.linked_app).unwrap_or(serde_json::Value::Null);
        }

        payload
    }

    // Fetch all categories for the user
    pub async fn fetch_categories(
        &self,
        access_token: &str,
    ) -> Result<Vec<RemoteCategory>, String> {
        let url = format!("{}/categories?select=*", self.rest_url());

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Fetch categories failed: {}", error_text));
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        serde_json::from_str::<Vec<RemoteCategory>>(&text)
            .map_err(|e| format!("Failed to parse categories: {} - Response was: {}", e, text))
    }

    // Fetch all todos for the user
    pub async fn fetch_todos(&self, access_token: &str) -> Result<Vec<RemoteTodo>, String> {
        let url = format!("{}/todos?select=*", self.rest_url());

        let response = self
            .client
            .get(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Fetch todos failed: {}", error_text));
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        serde_json::from_str::<Vec<RemoteTodo>>(&text)
            .map_err(|e| format!("Failed to parse todos: {} - Response was: {}", e, text))
    }

    // Upsert a category
    pub async fn upsert_category(
        &self,
        access_token: &str,
        category: &RemoteCategory,
    ) -> Result<RemoteCategory, String> {
        let url = format!("{}/categories", self.rest_url());

        let response = self
            .client
            .post(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header(
                "Prefer",
                "resolution=merge-duplicates,return=representation",
            )
            .json(category)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Upsert category failed: {}", error_text));
        }

        let mut categories: Vec<RemoteCategory> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        categories
            .pop()
            .ok_or_else(|| "No category returned".to_string())
    }

    // Upsert a todo
    pub async fn upsert_todo(
        &self,
        access_token: &str,
        todo: &RemoteTodo,
    ) -> Result<RemoteTodo, String> {
        let url = format!("{}/todos", self.rest_url());
        let mut include_reminder_at = true;
        let mut include_linked_app = true;

        loop {
            let payload = Self::build_todo_payload(todo, include_reminder_at, include_linked_app);

            let response = self
                .client
                .post(&url)
                .header("apikey", &self.config.anon_key)
                .header("Authorization", format!("Bearer {}", access_token))
                .header("Content-Type", "application/json")
                .header(
                    "Prefer",
                    "resolution=merge-duplicates,return=representation",
                )
                .json(&payload)
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;

            if response.status().is_success() {
                let mut todos: Vec<RemoteTodo> = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;

                return todos.pop().ok_or_else(|| "No todo returned".to_string());
            }

            let error_text = response.text().await.unwrap_or_default();

            if include_linked_app && error_text.contains("Could not find the 'linked_app' column") {
                include_linked_app = false;
                continue;
            }

            if include_reminder_at && error_text.contains("Could not find the 'reminder_at' column")
            {
                include_reminder_at = false;
                continue;
            }

            return Err(format!("Upsert todo failed: {}", error_text));
        }
    }

    // Delete a category
    pub async fn delete_category(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
        let url = format!("{}/categories?id=eq.{}", self.rest_url(), sync_id);

        let response = self
            .client
            .delete(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Delete category failed: {}", error_text));
        }

        Ok(())
    }

    // Delete a todo
    pub async fn delete_todo(&self, access_token: &str, sync_id: &str) -> Result<(), String> {
        let url = format!("{}/todos?id=eq.{}", self.rest_url(), sync_id);

        let response = self
            .client
            .delete(&url)
            .header("apikey", &self.config.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Delete todo failed: {}", error_text));
        }

        Ok(())
    }
}
