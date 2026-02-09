use std::{ffi::CString, mem::transmute};

use crate::c::macos::{general::{constants::RTLD_NOW, functions::{dlopen, dlsym}}, graphics::{appkit::NSUInteger, objc_runtime::Id}};

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

#[repr(transparent)]
pub struct MTLPixelFormat(NSUInteger);
// 8-bit
pub const INVALID: MTLPixelFormat = MTLPixelFormat(0);
pub const A8_UNORM: MTLPixelFormat = MTLPixelFormat(1);
pub const R8_UNORM: MTLPixelFormat = MTLPixelFormat(10);
pub const R8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(11);
pub const R8_SNORM: MTLPixelFormat = MTLPixelFormat(12);
pub const R8_UINT: MTLPixelFormat = MTLPixelFormat(13);
pub const R8_SINT: MTLPixelFormat = MTLPixelFormat(14);
// 16-bit
pub const R16_UNORM: MTLPixelFormat = MTLPixelFormat(20);
pub const R16_SNORM: MTLPixelFormat = MTLPixelFormat(22);
pub const R16_UINT: MTLPixelFormat = MTLPixelFormat(23);
pub const R16_SINT: MTLPixelFormat = MTLPixelFormat(24);
pub const R16_FLOAT: MTLPixelFormat = MTLPixelFormat(25);
pub const RG8_UNORM: MTLPixelFormat = MTLPixelFormat(30);
pub const RG8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(31);
pub const RG8_SNORM: MTLPixelFormat = MTLPixelFormat(32);
pub const RG8_UINT: MTLPixelFormat = MTLPixelFormat(33);
pub const RG8_SINT: MTLPixelFormat = MTLPixelFormat(34);
// 32-bit
pub const B5G6R5_UNORM: MTLPixelFormat = MTLPixelFormat(40);
pub const A1BGR5_UNORM: MTLPixelFormat = MTLPixelFormat(41);
pub const ABGR4_UNORM: MTLPixelFormat = MTLPixelFormat(42);
pub const BGR5A1_UNORM: MTLPixelFormat = MTLPixelFormat(43);
pub const R32_UINT: MTLPixelFormat = MTLPixelFormat(53);
pub const R32_SINT: MTLPixelFormat = MTLPixelFormat(54);
pub const R32_FLOAT: MTLPixelFormat = MTLPixelFormat(55);
pub const RG16_UNORM: MTLPixelFormat = MTLPixelFormat(60);
pub const RG16_SNORM: MTLPixelFormat = MTLPixelFormat(62);
pub const RG16_UINT: MTLPixelFormat = MTLPixelFormat(63);
pub const RG16_SINT: MTLPixelFormat = MTLPixelFormat(64);
pub const RG16_FLOAT: MTLPixelFormat = MTLPixelFormat(65);
// 32-bit RGBA
pub const RGBA8_UNORM: MTLPixelFormat = MTLPixelFormat(70);
pub const RGBA8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(71);
pub const RGBA8_SNORM: MTLPixelFormat = MTLPixelFormat(72);
pub const RGBA8_UINT: MTLPixelFormat = MTLPixelFormat(73);
pub const RGBA8_SINT: MTLPixelFormat = MTLPixelFormat(74);
// 32-bit BGRA
pub const BGRA8_UNORM: MTLPixelFormat = MTLPixelFormat(80);
pub const BGRA8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(81);
// Packed 32-bit
pub const RGB10A2_UNORM: MTLPixelFormat = MTLPixelFormat(90);
pub const RGB10A2_UINT: MTLPixelFormat = MTLPixelFormat(91);
pub const RG11B10_FLOAT: MTLPixelFormat = MTLPixelFormat(92);
pub const RGB9E5_FLOAT: MTLPixelFormat = MTLPixelFormat(93);
pub const BGR10A2_UNORM: MTLPixelFormat = MTLPixelFormat(94);
// 64-bit
pub const RG32_UINT: MTLPixelFormat = MTLPixelFormat(103);
pub const RG32_SINT: MTLPixelFormat = MTLPixelFormat(104);
pub const RG32_FLOAT: MTLPixelFormat = MTLPixelFormat(105);
pub const RGBA16_UNORM: MTLPixelFormat = MTLPixelFormat(110);
pub const RGBA16_SNORM: MTLPixelFormat = MTLPixelFormat(112);
pub const RGBA16_UINT: MTLPixelFormat = MTLPixelFormat(113);
pub const RGBA16_SINT: MTLPixelFormat = MTLPixelFormat(114);
pub const RGBA16_FLOAT: MTLPixelFormat = MTLPixelFormat(115);
// 128-bit
pub const RGBA32_UINT: MTLPixelFormat = MTLPixelFormat(123);
pub const RGBA32_SINT: MTLPixelFormat = MTLPixelFormat(124);
pub const RGBA32_FLOAT: MTLPixelFormat = MTLPixelFormat(125);
// Depth and Stencil
pub const DEPTH16_UNORM: MTLPixelFormat = MTLPixelFormat(250);
pub const DEPTH32_FLOAT: MTLPixelFormat = MTLPixelFormat(252);
pub const STENCIL8: MTLPixelFormat = MTLPixelFormat(253);
pub const DEPTH24_UNORM_STENCIL8: MTLPixelFormat = MTLPixelFormat(255);
pub const DEPTH32_FLOAT_STENCIL8: MTLPixelFormat = MTLPixelFormat(260);
pub const X32_STENCIL8: MTLPixelFormat = MTLPixelFormat(261);
pub const X24_STENCIL8: MTLPixelFormat = MTLPixelFormat(262);

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