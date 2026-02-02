use std::ffi::{c_char, c_void};

use crate::c::vulkan::functions::{PfnVkAllocationFunction, PfnVkFreeFunction, PfnVkInternalAllocationNotification, PfnVkInternalFreeNotification, PfnVkReallocationFunction};

pub type VkDeviceSize = u64;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkInstance(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDevice(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPhysicalDevice(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSurfaceKHR(pub *mut c_void);

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PfnVkAllocationFunction,
    pub pfn_reallocation: PfnVkReallocationFunction,
    pub pfn_free: PfnVkFreeFunction,
    pub pfn_internal_allocation: PfnVkInternalAllocationNotification,
    pub pfn_internal_free: PfnVkInternalFreeNotification
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub s_type: u32,
    pub p_next: *const c_void,
    pub flags: u32,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char
}