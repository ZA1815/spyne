#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageAspectFlagBits(pub u32);

impl std::ops::BitOr for VkImageAspectFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 0);
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 1);
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 2);
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1 << 3);
