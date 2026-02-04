#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkVertexInputRate(pub u32);

pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = VkVertexInputRate(0);
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = VkVertexInputRate(1);
