#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSharingMode(pub u32);

pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = VkSharingMode(0);
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = VkSharingMode(1);
