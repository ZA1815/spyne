use std::ffi::c_void;

use crate::c::vulkan::{constants::enums::structure_type::VkStructureType, types::base::VkFlags};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkShaderModule(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkShaderModuleCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}

