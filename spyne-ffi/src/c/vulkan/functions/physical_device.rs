use crate::c::vulkan::types::physical_device::{VkPhysicalDevice, VkPhysicalDeviceFeatures, VkPhysicalDeviceMemoryProperties, VkPhysicalDeviceProperties, VkQueueFamilyProperties};

pub type VkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_properties: *mut VkPhysicalDeviceProperties,
);

pub type VkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_features: *mut VkPhysicalDeviceFeatures,
);

pub type VkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_memory_properties: *mut VkPhysicalDeviceMemoryProperties,
);

pub type VkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut VkQueueFamilyProperties,
);

