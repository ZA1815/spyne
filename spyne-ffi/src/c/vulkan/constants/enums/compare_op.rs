#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCompareOp(pub u32);

pub const VK_COMPARE_OP_NEVER: VkCompareOp = VkCompareOp(0);
pub const VK_COMPARE_OP_LESS: VkCompareOp = VkCompareOp(1);
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = VkCompareOp(2);
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = VkCompareOp(3);
pub const VK_COMPARE_OP_GREATER: VkCompareOp = VkCompareOp(4);
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = VkCompareOp(5);
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = VkCompareOp(6);
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = VkCompareOp(7);
