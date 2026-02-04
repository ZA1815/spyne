#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageTiling(pub u32);

pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = VkImageTiling(0);
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = VkImageTiling(1);
