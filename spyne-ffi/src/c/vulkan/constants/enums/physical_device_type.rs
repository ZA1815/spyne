#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPhysicalDeviceType(pub u32);

pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(1);
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(2);
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(3);
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);
