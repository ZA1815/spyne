use std::{ffi::{CString, c_char, c_void}, mem::transmute};

use crate::c::macos::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}};

pub type Id = *mut c_void;
pub type Sel = *mut c_void;
pub type Class = *mut c_void;
pub type Bool = i8;

pub type ObjCGetClass = unsafe extern "C" fn(
    name: *const c_char
) -> Id;


pub type SelRegisterName = unsafe extern "C" fn(
    str: *const c_char
) -> Sel;

pub type ObjCMsgSend = unsafe extern "C" fn();

pub type ObjCAllocateClassPair = unsafe extern "C" fn(
    superclass: Class,
    name: *const c_char,
    extra_bytes: usize
) -> Class;

pub type ObjCRegisterClassPair = unsafe extern "C" fn(
    cls: Class
);

pub type Imp = unsafe extern "C" fn(Id, Sel, Id);

pub type ClassAddMethod = unsafe extern "C" fn(
    cls: Class,
    name: Sel,
    imp: Imp,
    types: *const c_char
) -> Bool;

pub struct ObjCFunctions {
    pub objc_get_class: ObjCGetClass,
    pub sel_register_name: SelRegisterName,
    pub objc_msg_send: ObjCMsgSend,
    pub objc_allocate_class_pair: ObjCAllocateClassPair,
    pub objc_register_class_pair: ObjCRegisterClassPair,
    pub class_add_method: ClassAddMethod
}

impl ObjCFunctions {
    pub unsafe fn load() -> Self {
        let lib = unsafe { dlopen(CString::new("libobjc.dylib").unwrap().as_ptr(), RTLD_NOW) };
        let objc_get_class: ObjCGetClass = unsafe { transmute(dlsym(lib, CString::new("objc_getClass").unwrap().as_ptr())) };
        let sel_register_name: SelRegisterName = unsafe { transmute(dlsym(lib, CString::new("sel_registerName").unwrap().as_ptr())) };
        let objc_msg_send: ObjCMsgSend = unsafe { transmute(dlsym(lib, CString::new("objc_msgSend").unwrap().as_ptr())) };
        let objc_allocate_class_pair: ObjCAllocateClassPair = unsafe { transmute(dlsym(lib, CString::new("objc_allocateClassPair").unwrap().as_ptr())) };
        let objc_register_class_pair: ObjCRegisterClassPair = unsafe { transmute(dlsym(lib, CString::new("objc_registerClassPair").unwrap().as_ptr())) };
        let class_add_method: ClassAddMethod = unsafe { transmute(dlsym(lib, CString::new("class_addMethod").unwrap().as_ptr())) };
        Self {
            objc_get_class,
            sel_register_name,
            objc_msg_send,
            objc_allocate_class_pair,
            objc_register_class_pair,
            class_add_method
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