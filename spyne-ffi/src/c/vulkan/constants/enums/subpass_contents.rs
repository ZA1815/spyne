#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSubpassContents(pub u32);

pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = VkSubpassContents(0);
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = VkSubpassContents(1);
