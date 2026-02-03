#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkShaderStageFlagBits(pub u32);

pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 0);
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 1);
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 2);
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 3);
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 4);
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1 << 5);
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(0x0000001F);
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(0x7FFFFFFF);
