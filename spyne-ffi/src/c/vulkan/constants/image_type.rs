#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageType(u32);

pub const VK_IMAGE_TYPE_1D: VkImageType = VkImageType(0);
pub const VK_IMAGE_TYPE_2D: VkImageType = VkImageType(1);
pub const VK_IMAGE_TYPE_3D: VkImageType = VkImageType(2);
