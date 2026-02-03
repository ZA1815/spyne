#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFenceCreateFlagBits(pub u32);

pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = VkFenceCreateFlagBits(1 << 0);
