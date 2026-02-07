#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkQueryControlFlagBits(pub u32);

impl std::ops::BitOr for VkQueryControlFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for VkQueryControlFlagBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = VkQueryControlFlagBits(1 << 0);
