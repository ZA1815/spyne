#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineBindPoint(u32);

pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = VkPipelineBindPoint(0);
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = VkPipelineBindPoint(1);
