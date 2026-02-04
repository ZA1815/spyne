#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkObjectType(pub u32);

pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = VkObjectType(0);
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = VkObjectType(1);
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = VkObjectType(2);
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = VkObjectType(3);
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = VkObjectType(4);
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = VkObjectType(5);
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = VkObjectType(6);
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = VkObjectType(7);
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = VkObjectType(8);
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = VkObjectType(9);
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = VkObjectType(10);
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = VkObjectType(11);
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = VkObjectType(12);
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = VkObjectType(13);
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = VkObjectType(14);
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = VkObjectType(15);
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = VkObjectType(16);
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = VkObjectType(17);
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = VkObjectType(18);
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = VkObjectType(19);
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = VkObjectType(20);
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = VkObjectType(21);
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = VkObjectType(22);
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = VkObjectType(23);
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = VkObjectType(24);
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = VkObjectType(25);
