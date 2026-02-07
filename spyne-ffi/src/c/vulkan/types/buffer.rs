use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{sharing_mode::VkSharingMode, structure_type::VkStructureType}, flags::{buffer_create::VkBufferCreateFlagBits, buffer_usage::VkBufferUsageFlagBits}}, types::base::VkDeviceSize};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkBuffer(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkBufferCreateFlagBits,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlagBits,
    pub sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

