#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageLayout(u32);

pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = VkImageLayout(0);
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = VkImageLayout(1);
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(2);
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(3);
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(4);
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(5);
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = VkImageLayout(6);
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = VkImageLayout(7);
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = VkImageLayout(8);
