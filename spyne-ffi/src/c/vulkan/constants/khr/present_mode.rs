#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPresentModeKHR(u32);

pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = VkPresentModeKHR(0);
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = VkPresentModeKHR(1);
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = VkPresentModeKHR(2);
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = VkPresentModeKHR(3);
