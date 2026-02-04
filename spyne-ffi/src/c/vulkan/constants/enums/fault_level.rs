#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFaultLevel(pub u32);

pub const VK_FAULT_LEVEL_UNASSIGNED: VkFaultLevel = VkFaultLevel(0);
pub const VK_FAULT_LEVEL_CRITICAL: VkFaultLevel = VkFaultLevel(1);
pub const VK_FAULT_LEVEL_RECOVERABLE: VkFaultLevel = VkFaultLevel(2);
pub const VK_FAULT_LEVEL_WARNING: VkFaultLevel = VkFaultLevel(3);
