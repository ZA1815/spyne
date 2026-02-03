#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCullModeFlagBits(pub u32);

pub const VK_CULL_MODE_NONE: VkCullModeFlagBits = VkCullModeFlagBits(0);
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1 << 0);
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1 << 1);
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(0x00000003);
