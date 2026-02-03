#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkIndexType(pub u32);

pub const VK_INDEX_TYPE_UINT16: VkIndexType = VkIndexType(0);
pub const VK_INDEX_TYPE_UINT32: VkIndexType = VkIndexType(1);
