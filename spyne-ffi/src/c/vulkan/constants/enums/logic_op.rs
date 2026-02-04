#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkLogicOp(pub u32);

pub const VK_LOGIC_OP_CLEAR: VkLogicOp = VkLogicOp(0);
pub const VK_LOGIC_OP_AND: VkLogicOp = VkLogicOp(1);
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = VkLogicOp(2);
pub const VK_LOGIC_OP_COPY: VkLogicOp = VkLogicOp(3);
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = VkLogicOp(4);
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = VkLogicOp(5);
pub const VK_LOGIC_OP_XOR: VkLogicOp = VkLogicOp(6);
pub const VK_LOGIC_OP_OR: VkLogicOp = VkLogicOp(7);
pub const VK_LOGIC_OP_NOR: VkLogicOp = VkLogicOp(8);
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = VkLogicOp(9);
pub const VK_LOGIC_OP_INVERT: VkLogicOp = VkLogicOp(10);
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = VkLogicOp(11);
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = VkLogicOp(12);
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = VkLogicOp(13);
pub const VK_LOGIC_OP_NAND: VkLogicOp = VkLogicOp(14);
pub const VK_LOGIC_OP_SET: VkLogicOp = VkLogicOp(15);
