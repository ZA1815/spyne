#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCompositeAlphaFlagBitsKHR(pub u32);

impl std::ops::BitOr for VkCompositeAlphaFlagBitsKHR {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 0);
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 1);
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 2);
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1 << 3);
