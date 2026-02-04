use std::ffi::c_void;

use std::ffi::c_char;

use crate::c::vulkan::{constants::enums::{fault_level::VkFaultLevel, fault_type::VkFaultType, structure_type::VkStructureType}, types::{base::VkFlags, physical_device::VkPhysicalDeviceFeatures}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDevice(pub *mut c_void);

#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: * const*const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: * const*const c_char,
    pub p_enabled_features: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
pub struct VkFaultData {
    pub s_type: VkStructureType,
    pub p_next: *mut c_void,
    pub fault_level: VkFaultLevel,
    pub fault_type: VkFaultType,
}

