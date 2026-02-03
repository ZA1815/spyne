#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageViewType(pub u32);

pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = VkImageViewType(0);
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = VkImageViewType(1);
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = VkImageViewType(2);
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = VkImageViewType(3);
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = VkImageViewType(4);
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = VkImageViewType(5);
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = VkImageViewType(6);
