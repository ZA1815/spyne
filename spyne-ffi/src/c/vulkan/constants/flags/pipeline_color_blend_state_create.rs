#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineColorBlendStateCreateFlagBits(pub u32);

impl std::ops::BitOr for VkPipelineColorBlendStateCreateFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

