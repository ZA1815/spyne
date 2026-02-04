#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkColorComponentFlagBits(pub u32);

impl std::ops::BitOr for VkColorComponentFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 0);
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 1);
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 2);
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1 << 3);
