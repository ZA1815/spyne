#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPolygonMode(pub u32);

pub const VK_POLYGON_MODE_FILL: VkPolygonMode = VkPolygonMode(0);
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = VkPolygonMode(1);
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = VkPolygonMode(2);
