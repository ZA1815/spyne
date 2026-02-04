use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{device_memory_report_event_type_ext::VkDeviceMemoryReportEventTypeEXT, object_type::VkObjectType, structure_type::VkStructureType}, flags::memory_property::VkMemoryPropertyFlagBits}, types::base::{VkDeviceSize, VkFlags}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceMemory(pub *mut c_void);

#[repr(C)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub allocation_size: VkDeviceSize,
    pub memory_type_index: u32,
}

#[repr(C)]
pub struct VkMemoryType {
    pub property_flags: VkMemoryPropertyFlagBits,
    pub heap_index: u32,
}

#[repr(C)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkFlags,
}

#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
    pub s_type: VkStructureType,
    pub p_next: *mut c_void,
    pub flags: VkFlags,
    pub r#type: VkDeviceMemoryReportEventTypeEXT,
    pub memory_object_id: u64,
    pub size: VkDeviceSize,
    pub object_type: VkObjectType,
    pub object_handle: u64,
    pub heap_index: u32,
}

