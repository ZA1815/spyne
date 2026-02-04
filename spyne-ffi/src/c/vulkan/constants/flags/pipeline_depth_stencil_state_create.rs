#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineDepthStencilStateCreateFlagBits(pub u32);

impl std::ops::BitOr for VkPipelineDepthStencilStateCreateFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

