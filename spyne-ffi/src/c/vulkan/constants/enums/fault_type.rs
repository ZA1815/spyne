#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFaultType(pub u32);

pub const VK_FAULT_TYPE_INVALID: VkFaultType = VkFaultType(0);
pub const VK_FAULT_TYPE_UNASSIGNED: VkFaultType = VkFaultType(1);
pub const VK_FAULT_TYPE_IMPLEMENTATION: VkFaultType = VkFaultType(2);
pub const VK_FAULT_TYPE_SYSTEM: VkFaultType = VkFaultType(3);
pub const VK_FAULT_TYPE_PHYSICAL_DEVICE: VkFaultType = VkFaultType(4);
pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL: VkFaultType = VkFaultType(5);
pub const VK_FAULT_TYPE_INVALID_API_USAGE: VkFaultType = VkFaultType(6);
