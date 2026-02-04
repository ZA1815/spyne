#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAccessFlagBits(pub u32);

impl std::ops::BitOr for VkAccessFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 0);
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 1);
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 2);
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 3);
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 4);
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 5);
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 6);
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 7);
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 8);
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 9);
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 10);
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 11);
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 12);
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 13);
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 14);
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 15);
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1 << 16);
