#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBlendOp(u32);

pub const VK_BLEND_OP_ADD: VkBlendOp = VkBlendOp(0);
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = VkBlendOp(1);
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = VkBlendOp(2);
pub const VK_BLEND_OP_MIN: VkBlendOp = VkBlendOp(3);
pub const VK_BLEND_OP_MAX: VkBlendOp = VkBlendOp(4);
