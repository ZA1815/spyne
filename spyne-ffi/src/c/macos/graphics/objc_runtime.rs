use std::{ffi::{CString, c_char, c_void}, mem::transmute};

use crate::c::macos::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}};

pub type Id = *mut c_void;

pub type ObjCGetClass = unsafe extern "C" fn(
    name: *const c_char
) -> Id;

pub type Sel = *mut c_void;

pub type SelRegisterName = unsafe extern "C" fn(
    str: *const c_char
) -> Sel;

pub type ObjCMsgSend = unsafe extern "C" fn();

pub struct ObjCFunctions {
    pub objc_get_class: ObjCGetClass,
    pub sel_register_name: SelRegisterName,
    pub objc_msg_send: ObjCMsgSend
}

impl ObjCFunctions {
    pub unsafe fn load() -> Self {
        let lib = unsafe { dlopen(CString::new("libobjc.dylib").unwrap().as_ptr(), RTLD_NOW) };
        let objc_get_class: ObjCGetClass = unsafe { transmute(dlsym(lib, CString::new("objc_getClass").unwrap().as_ptr())) };
        let sel_register_name: SelRegisterName = unsafe { transmute(dlsym(lib, CString::new("sel_registerName").unwrap().as_ptr())) };
        let objc_msg_send: ObjCMsgSend = unsafe { transmute(dlsym(lib, CString::new("objc_msgSend").unwrap().as_ptr())) };
        Self {
            objc_get_class,
            sel_register_name,
            objc_msg_send
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::CString;

    use crate::c::macos::graphics::objc_runtime::ObjCFunctions;

    #[test]
    fn test_objc_funcs() {
        let funcs = unsafe { ObjCFunctions::load() };
        let cls = unsafe { (funcs.objc_get_class)(CString::new("NSObject").unwrap().as_ptr()) };
        assert!(!cls.is_null())
    }
}