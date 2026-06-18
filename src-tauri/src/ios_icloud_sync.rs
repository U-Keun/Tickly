use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};

use tauri::{AppHandle, Runtime};

type ICloudSyncFn = unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);

const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;
const ICLOUD_SYNC_SYMBOL: &[u8] = b"tickly_perform_icloud_sync\0";
const FREE_STRING_SYMBOL: &[u8] = b"tickly_free_c_string\0";

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub fn perform_icloud_sync<R: Runtime>(
    _app: &AppHandle<R>,
    request_json: &str,
) -> Result<Option<String>, String> {
    let request_json = CString::new(request_json)
        .map_err(|_| "iCloud sync request contains invalid null bytes".to_string())?;
    let result = unsafe {
        match (resolve_icloud_sync_fn(), resolve_free_string_fn()) {
            (Some(sync), Some(free_string)) => {
                let pointer = sync(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    request_json.as_ptr(),
                );
                if pointer.is_null() {
                    None
                } else {
                    let value = CStr::from_ptr(pointer).to_string_lossy().to_string();
                    free_string(pointer);
                    Some(value)
                }
            }
            _ => None,
        }
    };

    Ok(result)
}

unsafe fn resolve_icloud_sync_fn() -> Option<ICloudSyncFn> {
    let symbol = unsafe { dlsym(RTLD_DEFAULT, ICLOUD_SYNC_SYMBOL.as_ptr().cast::<c_char>()) };
    if symbol.is_null() {
        return None;
    }

    Some(unsafe { std::mem::transmute::<*mut c_void, ICloudSyncFn>(symbol) })
}

unsafe fn resolve_free_string_fn() -> Option<FreeStringFn> {
    let symbol = unsafe { dlsym(RTLD_DEFAULT, FREE_STRING_SYMBOL.as_ptr().cast::<c_char>()) };
    if symbol.is_null() {
        return None;
    }

    Some(unsafe { std::mem::transmute::<*mut c_void, FreeStringFn>(symbol) })
}
