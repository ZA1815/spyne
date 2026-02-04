#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDependencyFlagBits(pub u32);

impl std::ops::BitOr for VkDependencyFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1 << 0);
