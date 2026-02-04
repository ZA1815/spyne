use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::structure_type::VkStructureType, flags::fence_create::VkFenceCreateFlagBits}, types::base::VkFlags};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkFence(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSemaphore(pub *mut c_void);

#[repr(C)]
pub struct VkFenceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFenceCreateFlagBits,
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
}

