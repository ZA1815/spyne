#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCommandBufferLevel(pub u32);

pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = VkCommandBufferLevel(0);
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = VkCommandBufferLevel(1);
