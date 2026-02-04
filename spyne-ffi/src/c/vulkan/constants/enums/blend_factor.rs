#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBlendFactor(pub u32);

pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = VkBlendFactor(0);
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = VkBlendFactor(1);
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = VkBlendFactor(2);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = VkBlendFactor(3);
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = VkBlendFactor(4);
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = VkBlendFactor(5);
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = VkBlendFactor(6);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = VkBlendFactor(7);
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = VkBlendFactor(8);
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = VkBlendFactor(9);
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(10);
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(11);
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(12);
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(13);
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = VkBlendFactor(14);
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = VkBlendFactor(15);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = VkBlendFactor(16);
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(17);
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(18);
