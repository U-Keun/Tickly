use std::{
    ffi::{c_void, CString},
    os::raw::c_char,
    sync::mpsc,
    time::Duration,
};

use tauri::{AppHandle, Manager, Runtime};

type ICloudSyncFn = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char) -> bool;

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;
const ICLOUD_SYNC_SYMBOL: &[u8] = b"tickly_exchange_icloud_sync\0";

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub fn exchange_icloud_sync<R: Runtime>(
    app: &AppHandle<R>,
    request_json: &str,
) -> Result<bool, String> {
    let request_json = CString::new(request_json)
        .map_err(|_| "iCloud sync request contains invalid null bytes".to_string())?;
    let Some(webview_window) = app.get_webview_window("main") else {
        return Ok(false);
    };

    let (sender, receiver) = mpsc::channel();
    webview_window
        .with_webview(move |webview| unsafe {
            let did_start = resolve_icloud_sync_fn()
                .map(|exchange| {
                    exchange(
                        webview.inner(),
                        webview.view_controller(),
                        request_json.as_ptr(),
                    )
                })
                .unwrap_or(false);
            let _ = sender.send(did_start);
        })
        .map_err(|error| error.to_string())?;

    Ok(receiver
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or(false))
}

unsafe fn resolve_icloud_sync_fn() -> Option<ICloudSyncFn> {
    let symbol = unsafe { dlsym(RTLD_DEFAULT, ICLOUD_SYNC_SYMBOL.as_ptr().cast::<c_char>()) };
    if symbol.is_null() {
        return None;
    }

    Some(unsafe { std::mem::transmute::<*mut c_void, ICloudSyncFn>(symbol) })
}
