#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAttachmentStoreOp(pub u32);

pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = VkAttachmentStoreOp(0);
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = VkAttachmentStoreOp(1);
