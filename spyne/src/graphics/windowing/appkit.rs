use std::{ffi::CString, mem::transmute, ptr::{null, null_mut}};

use spyne_ffi::c::macos::graphics::{appkit::{NS_BACKING_STORE_BUFFERED, NS_WINDOW_STYLE_MASK_CLOSABLE, NS_WINDOW_STYLE_MASK_MINIATURIZABLE, NS_WINDOW_STYLE_MASK_RESIZABLE, NS_WINDOW_STYLE_MASK_TITLED, NSBackingStoreType, NSPoint, NSRect, NSSize, NSWindowStyleMask}, objc_runtime::{Id, ObjCFunctions, Sel}};

pub struct AppKitWindow {
    functions: ObjCFunctions
}

impl AppKitWindow {
    pub fn create_window() {
        let functions = unsafe { ObjCFunctions::load() };
        
        let nsapp_class = unsafe { (functions.objc_get_class)(CString::new("NSApplication").unwrap().as_ptr()) };
        let nsapp_sel = unsafe { (functions.sel_register_name)(CString::new("sharedApplication").unwrap().as_ptr()) };
        let nsapp_msg_send: unsafe extern "C" fn(Id, Sel) -> Id = unsafe { transmute(functions.objc_msg_send) };
        let nsapp_ptr = unsafe { nsapp_msg_send(nsapp_class, nsapp_sel) };
        if nsapp_ptr.is_null() {
            panic!("Got a null ptr on nsapp");
        }
        
        let sap_sel = unsafe { (functions.sel_register_name)(CString::new("setActivationPolicy:").unwrap().as_ptr()) };
        let sap_msg_send: unsafe extern "C" fn(Id, Sel, i64) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { sap_msg_send(nsapp_ptr, sap_sel, 0) };
        
        let nswindow_class = unsafe { (functions.objc_get_class)(CString::new("NSWindow").unwrap().as_ptr()) };
        let nswindow_sel = unsafe { (functions.sel_register_name)(CString::new("alloc").unwrap().as_ptr()) };
        let nswindow_msg_send: unsafe extern "C" fn(Id, Sel) -> Id = unsafe { transmute(functions.objc_msg_send) };
        let nswindow_ptr = unsafe { nswindow_msg_send(nswindow_class, nswindow_sel) };
        if nswindow_ptr.is_null() {
            panic!("Got a null ptr on nswindow");
        }
        
        let iwcr_sel = unsafe { (functions.sel_register_name)(CString::new("initWithContentRect:styleMask:backing:defer:").unwrap().as_ptr()) };
        let iwcr_msg_send: unsafe extern "C" fn(Id, Sel, NSRect, NSWindowStyleMask, NSBackingStoreType, bool) -> Id = unsafe { transmute(functions.objc_msg_send) };
        let ns_rect = NSRect {
            origin: NSPoint {
                x: 100.0,
                y: 100.0,
            },
            size: NSSize {
                width: 800.0,
                height: 600.0,
            }
        };
        let style_mask: NSWindowStyleMask = NS_WINDOW_STYLE_MASK_TITLED | NS_WINDOW_STYLE_MASK_CLOSABLE | NS_WINDOW_STYLE_MASK_MINIATURIZABLE | NS_WINDOW_STYLE_MASK_RESIZABLE;
        let iwcr_ptr = unsafe { iwcr_msg_send(nswindow_ptr, iwcr_sel, ns_rect, style_mask, NS_BACKING_STORE_BUFFERED, false) };
        if iwcr_ptr.is_null() {
            panic!("Got a null ptr on iwcr");
        }
        
        let cv_sel = unsafe { (functions.sel_register_name)(CString::new("contentView").unwrap().as_ptr()) };
        let cv_msg_send: unsafe extern "C" fn(Id, Sel) -> Id = unsafe { transmute(functions.objc_msg_send) };
        let cv_ptr = unsafe { cv_msg_send(nswindow_ptr, cv_sel) };
        if cv_ptr.is_null() {
            panic!("Got a null ptr on cv");
        }
        
        let swl_sel = unsafe { (functions.sel_register_name)(CString::new("setWantsLayer:").unwrap().as_ptr()) };
        let swl_msg_send: unsafe extern "C" fn(Id, Sel, bool) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { swl_msg_send(cv_ptr, swl_sel, true) };
        
        let caml_class = unsafe { (functions.objc_get_class)(CString::new("CAMetalLayer").unwrap().as_ptr()) };
        let caml_sel = unsafe { (functions.sel_register_name)(CString::new("layer").unwrap().as_ptr()) };
        let caml_msg_send: unsafe extern "C" fn(Id, Sel) -> Id = unsafe { transmute(functions.objc_msg_send) };
        let caml_ptr = unsafe { caml_msg_send(caml_class, caml_sel) };
        if caml_ptr.is_null() {
            panic!("Got a null ptr on caml");
        }
        
        let sl_sel = unsafe { (functions.sel_register_name)(CString::new("setLayer:").unwrap().as_ptr()) };
        let sl_msg_send: unsafe extern "C" fn(Id, Sel, Id) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { sl_msg_send(cv_ptr, sl_sel, caml_ptr) };
        
        let mkaof_sel = unsafe { (functions.sel_register_name)(CString::new("makeKeyAndOrderFront:").unwrap().as_ptr()) };
        let mkaof_msg_send: unsafe extern "C" fn(Id, Sel, Id) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { mkaof_msg_send(nswindow_ptr, mkaof_sel, null_mut()) };
        
        let aioa_sel = unsafe { (functions.sel_register_name)(CString::new("activateIgnoringOtherApps").unwrap().as_ptr()) };
        let aioa_msg_send: unsafe extern "C" fn(Id, Sel, bool) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { aioa_msg_send(nsapp_ptr, aioa_sel, true) };
        
        let run_sel = unsafe { (functions.sel_register_name)(CString::new("run").unwrap().as_ptr()) };
        let run_msg_send: unsafe extern "C" fn(Id, Sel) = unsafe { transmute(functions.objc_msg_send) };
        unsafe { run_msg_send(nsapp_ptr, run_sel) };
    }
}

#[cfg(test)]
mod test {
    use crate::graphics::windowing::appkit::AppKitWindow;

    #[test]
    fn test_windowing_appkit() {
        AppKitWindow::create_window();
    }
}