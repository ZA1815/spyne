use std::ffi::c_char;

use crate::c::vulkan::{constants::enums::result::VkResult, functions::func_pointers::PfnVkVoidFunction, types::{device::{VkDevice, VkDeviceCreateInfo}, instance::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo}, physical_device::VkPhysicalDevice}};

pub type VkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut VkInstance,
) -> VkResult;

pub type VkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: VkInstance,
    p_name: *const c_char,
) -> PfnVkVoidFunction;

pub type VkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut VkPhysicalDevice,
) -> VkResult;

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

