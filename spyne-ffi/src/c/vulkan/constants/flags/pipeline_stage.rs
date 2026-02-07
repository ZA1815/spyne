#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineStageFlagBits(pub u32);

impl std::ops::BitOr for VkPipelineStageFlagBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for VkPipelineStageFlagBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 0);
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 1);
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 2);
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 3);
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 4);
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 5);
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 6);
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 7);
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 8);
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 9);
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 10);
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 11);
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 12);
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 13);
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 14);
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 15);
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1 << 16);
