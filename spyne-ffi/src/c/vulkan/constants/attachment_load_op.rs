#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAttachmentLoadOp(u32);

pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = VkAttachmentLoadOp(0);
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = VkAttachmentLoadOp(1);
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = VkAttachmentLoadOp(2);
