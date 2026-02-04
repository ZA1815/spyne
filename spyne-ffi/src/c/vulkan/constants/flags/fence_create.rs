#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFenceCreateFlagBits(pub u32);

impl std::ops::BitOr for VkFenceCreateFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = VkFenceCreateFlagBits(1 << 0);
