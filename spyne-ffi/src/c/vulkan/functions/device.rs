use std::ffi::c_char;

use crate::c::vulkan::{constants::enums::result::VkResult, functions::func_pointers::PfnVkVoidFunction, types::{device::{VkDevice, VkDeviceCreateInfo}, instance::VkAllocationCallbacks, physical_device::VkPhysicalDevice, queue::VkQueue}};

pub type VkCreateDevice = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_create_info: *const VkDeviceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_device: *mut VkDevice,
) -> VkResult;

pub type VkGetDeviceProcAddr = unsafe extern "system" fn(
    device: VkDevice,
    p_name: *const c_char,
) -> PfnVkVoidFunction;

pub type VkGetDeviceQueue = unsafe extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut VkQueue,
);

pub type VkDeviceWaitIdle = unsafe extern "system" fn(
    device: VkDevice,
) -> VkResult;

pub type VkDestroyDevice = unsafe extern "system" fn(
    device: VkDevice,
    p_allocator: *const VkAllocationCallbacks,
);

