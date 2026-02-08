use std::{ffi::CString, mem::transmute};

use crate::c::macos::{general::{constants::RTLD_NOW, functions::{dlopen, dlsym}}, graphics::objc_runtime::Id};

pub type MTLCreateSystemDefaultDevice = unsafe extern "C" fn() -> Id;

pub struct MetalFunctions {
    pub mtl_create_system_default_device: MTLCreateSystemDefaultDevice
}

impl MetalFunctions {
    pub unsafe fn load() -> Self {
        let lib = unsafe { dlopen(CString::new("/System/Library/Frameworks/Metal.framework/Metal").unwrap().as_ptr(), RTLD_NOW) };
        let mtl_create_system_default_device: MTLCreateSystemDefaultDevice = unsafe { transmute(dlsym(lib, CString::new("MTLCreateSystemDefaultDevice").unwrap().as_ptr())) };
        Self {
            mtl_create_system_default_device
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ffi::CString, mem::transmute};

    use crate::c::macos::graphics::{metal::MetalFunctions, objc_runtime::{Id, ObjCFunctions, Sel}};

    #[test]
    fn test_metal_funcs() {
        let funcs = unsafe { MetalFunctions::load() };
        let device_ptr = unsafe { (funcs.mtl_create_system_default_device)() };
        assert!(!device_ptr.is_null())
    }
    
    #[test]
    fn get_new_command_queue() {
        let objc_funcs = unsafe { ObjCFunctions::load() };
        let ncq_selector = unsafe { (objc_funcs.sel_register_name)(CString::new("newCommandQueue").unwrap().as_ptr()) };
        let metal_funcs = unsafe { MetalFunctions::load() };
        let device_ptr = unsafe { (metal_funcs.mtl_create_system_default_device)() };
        let msg_send: unsafe extern "C" fn(Id, Sel) -> Id = unsafe { transmute(objc_funcs.objc_msg_send) };
        let ncq_ptr = unsafe { msg_send(device_ptr, ncq_selector) };
        assert!(!ncq_ptr.is_null())
    }
}