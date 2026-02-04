#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAttachmentDescriptionFlagBits(pub u32);

impl std::ops::BitOr for VkAttachmentDescriptionFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlagBits(1 << 0);
