#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkStencilOp(pub u32);

pub const VK_STENCIL_OP_KEEP: VkStencilOp = VkStencilOp(0);
pub const VK_STENCIL_OP_ZERO: VkStencilOp = VkStencilOp(1);
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = VkStencilOp(2);
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(3);
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(4);
pub const VK_STENCIL_OP_INVERT: VkStencilOp = VkStencilOp(5);
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = VkStencilOp(6);
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = VkStencilOp(7);
