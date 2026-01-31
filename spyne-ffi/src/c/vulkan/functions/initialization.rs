use std::ffi::{c_char, c_void};

use crate::c::vulkan::types::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo, VkPhysicalDevice};


pub type PfnVkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: u32
) -> *mut c_void;

pub type PfnVkReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: u32
) -> *mut c_void;

pub type PfnVkFreeFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_memory: *mut c_void
);

pub type PfnVkInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: u32,
    allocation_scope: u32
);

pub type PfnVkInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: u32,
    allocation_scope: u32
);

pub type PfnVkVoidFunction = unsafe extern "system" fn();

pub type VkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut VkInstance
) -> i32;

pub type VkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: VkPhysicalDevice
) -> i32;

pub type VkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: VkInstance,
    p_name: *const c_char
) -> PfnVkVoidFunction;