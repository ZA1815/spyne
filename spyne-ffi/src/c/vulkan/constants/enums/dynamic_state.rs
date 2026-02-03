#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDynamicState(pub u32);

pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = VkDynamicState(0);
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = VkDynamicState(1);
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = VkDynamicState(2);
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = VkDynamicState(3);
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = VkDynamicState(4);
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = VkDynamicState(5);
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = VkDynamicState(6);
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = VkDynamicState(7);
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = VkDynamicState(8);
