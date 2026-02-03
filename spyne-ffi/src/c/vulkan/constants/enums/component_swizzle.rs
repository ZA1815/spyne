#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkComponentSwizzle(pub u32);

pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = VkComponentSwizzle(0);
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = VkComponentSwizzle(1);
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = VkComponentSwizzle(2);
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = VkComponentSwizzle(3);
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = VkComponentSwizzle(4);
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = VkComponentSwizzle(5);
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = VkComponentSwizzle(6);
