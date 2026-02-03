#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageUsageFlagBits(pub u32);

pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 0);
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 1);
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 2);
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 3);
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 4);
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 5);
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 6);
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1 << 7);
