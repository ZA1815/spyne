use std::{ffi::CString, mem::transmute, ops::BitOr};

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

#[repr(C)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize
}

#[repr(C)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger
}

#[repr(C)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger
}

#[repr(C)]
pub struct MTLClearColor {
    pub alpha: f64,
    pub blue: f64,
    pub green: f64,
    pub red: f64
}

#[repr(transparent)]
pub struct MTLPixelFormat(NSUInteger);
// 8-bit
pub const MTL_PIXEL_FORMAT_INVALID: MTLPixelFormat = MTLPixelFormat(0);
pub const MTL_PIXEL_FORMAT_A8_UNORM: MTLPixelFormat = MTLPixelFormat(1);
pub const MTL_PIXEL_FORMAT_R8_UNORM: MTLPixelFormat = MTLPixelFormat(10);
pub const MTL_PIXEL_FORMAT_R8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(11);
pub const MTL_PIXEL_FORMAT_R8_SNORM: MTLPixelFormat = MTLPixelFormat(12);
pub const MTL_PIXEL_FORMAT_R8_UINT: MTLPixelFormat = MTLPixelFormat(13);
pub const MTL_PIXEL_FORMAT_R8_SINT: MTLPixelFormat = MTLPixelFormat(14);
// 16-bit
pub const MTL_PIXEL_FORMAT_R16_UNORM: MTLPixelFormat = MTLPixelFormat(20);
pub const MTL_PIXEL_FORMAT_R16_SNORM: MTLPixelFormat = MTLPixelFormat(22);
pub const MTL_PIXEL_FORMAT_R16_UINT: MTLPixelFormat = MTLPixelFormat(23);
pub const MTL_PIXEL_FORMAT_R16_SINT: MTLPixelFormat = MTLPixelFormat(24);
pub const MTL_PIXEL_FORMAT_R16_FLOAT: MTLPixelFormat = MTLPixelFormat(25);
pub const MTL_PIXEL_FORMAT_RG8_UNORM: MTLPixelFormat = MTLPixelFormat(30);
pub const MTL_PIXEL_FORMAT_RG8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(31);
pub const MTL_PIXEL_FORMAT_RG8_SNORM: MTLPixelFormat = MTLPixelFormat(32);
pub const MTL_PIXEL_FORMAT_RG8_UINT: MTLPixelFormat = MTLPixelFormat(33);
pub const MTL_PIXEL_FORMAT_RG8_SINT: MTLPixelFormat = MTLPixelFormat(34);
// 32-bit
pub const MTL_PIXEL_FORMAT_B5G6R5_UNORM: MTLPixelFormat = MTLPixelFormat(40);
pub const MTL_PIXEL_FORMAT_A1BGR5_UNORM: MTLPixelFormat = MTLPixelFormat(41);
pub const MTL_PIXEL_FORMAT_ABGR4_UNORM: MTLPixelFormat = MTLPixelFormat(42);
pub const MTL_PIXEL_FORMAT_BGR5A1_UNORM: MTLPixelFormat = MTLPixelFormat(43);
pub const MTL_PIXEL_FORMAT_R32_UINT: MTLPixelFormat = MTLPixelFormat(53);
pub const MTL_PIXEL_FORMAT_R32_SINT: MTLPixelFormat = MTLPixelFormat(54);
pub const MTL_PIXEL_FORMAT_R32_FLOAT: MTLPixelFormat = MTLPixelFormat(55);
pub const MTL_PIXEL_FORMAT_RG16_UNORM: MTLPixelFormat = MTLPixelFormat(60);
pub const MTL_PIXEL_FORMAT_RG16_SNORM: MTLPixelFormat = MTLPixelFormat(62);
pub const MTL_PIXEL_FORMAT_RG16_UINT: MTLPixelFormat = MTLPixelFormat(63);
pub const MTL_PIXEL_FORMAT_RG16_SINT: MTLPixelFormat = MTLPixelFormat(64);
pub const MTL_PIXEL_FORMAT_RG16_FLOAT: MTLPixelFormat = MTLPixelFormat(65);
// 32-bit
pub const MTL_PIXEL_FORMAT_RGBA8_UNORM: MTLPixelFormat = MTLPixelFormat(70);
pub const MTL_PIXEL_FORMAT_RGBA8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(71);
pub const MTL_PIXEL_FORMAT_RGBA8_SNORM: MTLPixelFormat = MTLPixelFormat(72);
pub const MTL_PIXEL_FORMAT_RGBA8_UINT: MTLPixelFormat = MTLPixelFormat(73);
pub const MTL_PIXEL_FORMAT_RGBA8_SINT: MTLPixelFormat = MTLPixelFormat(74);
// 32-bit
pub const MTL_PIXEL_FORMAT_BGRA8_UNORM: MTLPixelFormat = MTLPixelFormat(80);
pub const MTL_PIXEL_FORMAT_BGRA8_UNORM_SRGB: MTLPixelFormat = MTLPixelFormat(81);
// Packed 32-bit
pub const MTL_PIXEL_FORMAT_RGB10A2_UNORM: MTLPixelFormat = MTLPixelFormat(90);
pub const MTL_PIXEL_FORMAT_RGB10A2_UINT: MTLPixelFormat = MTLPixelFormat(91);
pub const MTL_PIXEL_FORMAT_RG11B10_FLOAT: MTLPixelFormat = MTLPixelFormat(92);
pub const MTL_PIXEL_FORMAT_RGB9E5_FLOAT: MTLPixelFormat = MTLPixelFormat(93);
pub const MTL_PIXEL_FORMAT_BGR10A2_UNORM: MTLPixelFormat = MTLPixelFormat(94);
// 64-bit
pub const MTL_PIXEL_FORMAT_RG32_UINT: MTLPixelFormat = MTLPixelFormat(103);
pub const MTL_PIXEL_FORMAT_RG32_SINT: MTLPixelFormat = MTLPixelFormat(104);
pub const MTL_PIXEL_FORMAT_RG32_FLOAT: MTLPixelFormat = MTLPixelFormat(105);
pub const MTL_PIXEL_FORMAT_RGBA16_UNORM: MTLPixelFormat = MTLPixelFormat(110);
pub const MTL_PIXEL_FORMAT_RGBA16_SNORM: MTLPixelFormat = MTLPixelFormat(112);
pub const MTL_PIXEL_FORMAT_RGBA16_UINT: MTLPixelFormat = MTLPixelFormat(113);
pub const MTL_PIXEL_FORMAT_RGBA16_SINT: MTLPixelFormat = MTLPixelFormat(114);
pub const MTL_PIXEL_FORMAT_RGBA16_FLOAT: MTLPixelFormat = MTLPixelFormat(115);
// 128-bit
pub const MTL_PIXEL_FORMAT_RGBA32_UINT: MTLPixelFormat = MTLPixelFormat(123);
pub const MTL_PIXEL_FORMAT_RGBA32_SINT: MTLPixelFormat = MTLPixelFormat(124);
pub const MTL_PIXEL_FORMAT_RGBA32_FLOAT: MTLPixelFormat = MTLPixelFormat(125);
// Depth and Stencil
pub const MTL_PIXEL_FORMAT_DEPTH16_UNORM: MTLPixelFormat = MTLPixelFormat(250);
pub const MTL_PIXEL_FORMAT_DEPTH32_FLOAT: MTLPixelFormat = MTLPixelFormat(252);
pub const MTL_PIXEL_FORMAT_STENCIL8: MTLPixelFormat = MTLPixelFormat(253);
pub const MTL_PIXEL_FORMAT_DEPTH24_UNORM_STENCIL8: MTLPixelFormat = MTLPixelFormat(255);
pub const MTL_PIXEL_FORMAT_DEPTH32_FLOAT_STENCIL8: MTLPixelFormat = MTLPixelFormat(260);
pub const MTL_PIXEL_FORMAT_X32_STENCIL8: MTLPixelFormat = MTLPixelFormat(261);
pub const MTL_PIXEL_FORMAT_X24_STENCIL8: MTLPixelFormat = MTLPixelFormat(262);

#[repr(transparent)]
pub struct MTLStoreAction(NSUInteger);
pub const MTL_STORE_ACTION_DONT_CARE: MTLStoreAction = MTLStoreAction(0);
pub const MTL_STORE_ACTION_STORE: MTLStoreAction = MTLStoreAction(1);
pub const MTL_STORE_ACTION_MULTISAMPLE_RESOLVE: MTLStoreAction = MTLStoreAction(2);
pub const MTL_STORE_ACTION_STORE_AND_MULTISAMPLE_RESOLVE: MTLStoreAction = MTLStoreAction(3);
pub const MTL_STORE_ACTION_UNKNOWN: MTLStoreAction = MTLStoreAction(4);
pub const MTL_STORE_ACTION_CUSTOM_SAMPLE_DEPTH_STORE: MTLStoreAction = MTLStoreAction(5);

#[repr(transparent)]
pub struct MTLLoadAction(NSUInteger);
pub const MTL_LOAD_ACTION_DONT_CARE: MTLLoadAction = MTLLoadAction(0);
pub const MTL_LOAD_ACTION_LOAD: MTLLoadAction = MTLLoadAction(1);
pub const MTL_LOAD_ACTION_CLEAR: MTLLoadAction = MTLLoadAction(2);

#[repr(transparent)]
pub struct MTLPrimitiveType(NSUInteger);
pub const MTL_PRIMITIVE_TYPE_POINT: MTLPrimitiveType = MTLPrimitiveType(0);
pub const MTL_PRIMITIVE_TYPE_LINE: MTLPrimitiveType = MTLPrimitiveType(1);
pub const MTL_PRIMITIVE_TYPE_LINE_STRIP: MTLPrimitiveType = MTLPrimitiveType(2);
pub const MTL_PRIMITIVE_TYPE_TRIANGLE: MTLPrimitiveType = MTLPrimitiveType(3);
pub const MTL_PRIMITIVE_TYPE_TRIANGLE_STRIP: MTLPrimitiveType = MTLPrimitiveType(4);

#[repr(transparent)]
pub struct MTLTextureType(NSUInteger);
pub const MTL_TEXTURE_TYPE_1D: MTLTextureType = MTLTextureType(0);
pub const MTL_TEXTURE_TYPE_1D_ARRAY: MTLTextureType = MTLTextureType(1);
pub const MTL_TEXTURE_TYPE_2D: MTLTextureType = MTLTextureType(2);
pub const MTL_TEXTURE_TYPE_2D_ARRAY: MTLTextureType = MTLTextureType(3);
pub const MTL_TEXTURE_TYPE_2D_MULTISAMPLE: MTLTextureType = MTLTextureType(4);
pub const MTL_TEXTURE_TYPE_CUBE: MTLTextureType = MTLTextureType(5);
pub const MTL_TEXTURE_TYPE_CUBE_ARRAY: MTLTextureType = MTLTextureType(6);
pub const MTL_TEXTURE_TYPE_3D: MTLTextureType = MTLTextureType(7);
pub const MTL_TEXTURE_TYPE_2D_MULTISAMPLE_ARRAY: MTLTextureType = MTLTextureType(8);
pub const MTL_TEXTURE_TYPE_TEXTURE_BUFFER: MTLTextureType = MTLTextureType(9);

#[repr(transparent)]
pub struct MTLResourceOptions(NSUInteger);
impl BitOr for MTLResourceOptions {
    type Output = MTLResourceOptions;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
// CPU Cache
pub const MTL_RESOURCE_CPU_CACHE_MODE_DEFAULT_CACHE: MTLResourceOptions = MTLResourceOptions(0 << 0);
pub const MTL_RESOURCE_CPU_CACHE_MODE_WRITE_COMBINED: MTLResourceOptions = MTLResourceOptions(1 << 0);
// Storage
pub const MTL_RESOURCE_STORAGE_MODE_SHARED: MTLResourceOptions = MTLResourceOptions(0 << 4);
pub const MTL_RESOURCE_STORAGE_MODE_MANAGED: MTLResourceOptions = MTLResourceOptions(1 << 4);
pub const MTL_RESOURCE_STORAGE_MODE_PRIVATE: MTLResourceOptions = MTLResourceOptions(2 << 4);
pub const MTL_RESOURCE_STORAGE_MODE_MEMORYLESS: MTLResourceOptions = MTLResourceOptions(3 << 4);
// Hazard Tracking
pub const MTL_RESOURCE_HAZARD_TRACKING_MODE_DEFAULT: MTLResourceOptions = MTLResourceOptions(0 << 8);
pub const MTL_RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED: MTLResourceOptions = MTLResourceOptions(1 << 8);
pub const MTL_RESOURCE_HAZARD_TRACKING_MODE_TRACKED: MTLResourceOptions = MTLResourceOptions(2 << 8);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MTLSamplerMinMagFilter(NSUInteger);
pub const MTL_SAMPLER_MIN_MAG_FILTER_NEAREST: MTLSamplerMinMagFilter = MTLSamplerMinMagFilter(0);
pub const MTL_SAMPLER_MIN_MAG_FILTER_LINEAR: MTLSamplerMinMagFilter = MTLSamplerMinMagFilter(1);

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