#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPrimitiveTopology(pub u32);

pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = VkPrimitiveTopology(0);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(1);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(2);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(3);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(4);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = VkPrimitiveTopology(5);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(6);
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(7);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(8);
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(9);
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = VkPrimitiveTopology(10);
