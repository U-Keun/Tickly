use std::ffi::c_void;

use objc2::{msg_send, runtime::AnyObject};
use objc2_ui_kit::{
    UIEdgeInsets, UIScrollView, UIScrollViewContentInsetAdjustmentBehavior, UIView,
    UIViewAutoresizing, UIViewController,
};
use tauri::{Manager, Runtime};

pub fn configure_ios_fullscreen_viewport<R: Runtime>(app: &tauri::App<R>) {
    let Some(webview_window) = app.get_webview_window("main") else {
        log::warn!("Unable to configure iOS fullscreen viewport: main webview missing");
        return;
    };

    if let Err(error) = webview_window.with_webview(|webview| unsafe {
        configure_webview(webview.inner(), webview.view_controller());
    }) {
        log::warn!("Unable to configure iOS fullscreen viewport: {}", error);
    }
}

unsafe fn configure_webview(webview_ptr: *mut c_void, view_controller_ptr: *mut c_void) {
    if webview_ptr.is_null() {
        return;
    }

    let webview = unsafe { &*(webview_ptr.cast::<UIView>()) };
    configure_view(webview);

    if let Some(superview) = webview.superview() {
        configure_view(&superview);
        webview.setFrame(superview.bounds());
    }

    configure_scroll_view(webview_ptr.cast::<AnyObject>());

    if view_controller_ptr.is_null() {
        return;
    }

    let view_controller = unsafe { &*(view_controller_ptr.cast::<UIViewController>()) };
    if let Some(root_view) = view_controller.view() {
        configure_view(&root_view);
        if let Some(superview) = root_view.superview() {
            configure_view(&superview);
            root_view.setFrame(superview.bounds());
        }
        webview.setFrame(root_view.bounds());
    }
}

fn configure_view(view: &UIView) {
    view.setAutoresizingMask(
        UIViewAutoresizing::FlexibleWidth | UIViewAutoresizing::FlexibleHeight,
    );
    view.setInsetsLayoutMarginsFromSafeArea(false);
    view.setClipsToBounds(false);
}

unsafe fn configure_scroll_view(webview: *mut AnyObject) {
    let scroll_view: *mut UIScrollView = unsafe { msg_send![webview, scrollView] };
    if scroll_view.is_null() {
        return;
    }

    let scroll_view = unsafe { &*scroll_view };
    let zero_insets = UIEdgeInsets {
        top: 0.0,
        left: 0.0,
        bottom: 0.0,
        right: 0.0,
    };

    scroll_view
        .setContentInsetAdjustmentBehavior(UIScrollViewContentInsetAdjustmentBehavior::Never);
    scroll_view.setAutomaticallyAdjustsScrollIndicatorInsets(false);
    scroll_view.setContentInset(zero_insets);
    scroll_view.setScrollIndicatorInsets(zero_insets);
}
