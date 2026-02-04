#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageCreateFlagBits(pub u32);

impl std::ops::BitOr for VkImageCreateFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 0);
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 1);
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 2);
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 3);
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1 << 4);
