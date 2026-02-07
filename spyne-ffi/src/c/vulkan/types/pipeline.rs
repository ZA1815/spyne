use std::ffi::c_void;

use std::ffi::c_char;

use crate::c::vulkan::{constants::{enums::{blend_factor::VkBlendFactor, blend_op::VkBlendOp, compare_op::VkCompareOp, dynamic_state::VkDynamicState, format::VkFormat, front_face::VkFrontFace, logic_op::VkLogicOp, polygon_mode::VkPolygonMode, primitive_topology::VkPrimitiveTopology, stencil_op::VkStencilOp, structure_type::VkStructureType, vertex_input_rate::VkVertexInputRate}, flags::{color_component::VkColorComponentFlagBits, cull_mode::VkCullModeFlagBits, pipeline_color_blend_state_create::VkPipelineColorBlendStateCreateFlagBits, pipeline_create::VkPipelineCreateFlagBits, pipeline_depth_stencil_state_create::VkPipelineDepthStencilStateCreateFlagBits, pipeline_layout_create::VkPipelineLayoutCreateFlagBits, pipeline_shader_stage_create::VkPipelineShaderStageCreateFlagBits, sample_count::VkSampleCountFlagBits, shader_stage::VkShaderStageFlagBits}}, types::{base::{VkBool32, VkFlags, VkSampleMask}, image::VkExtent2D, render_pass::VkRenderPass, shader::VkShaderModule}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPipeline(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineCache(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineLayout(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDescriptorSetLayout(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineLayoutCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkPipelineLayoutCreateFlagBits,
    pub set_layout_count: u32,
    pub p_set_layouts: *const VkDescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const VkPushConstantRange,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineShaderStageCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlagBits,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const VkSpecializationInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VkVertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub topology: VkPrimitiveTopology,
    pub primitive_restart_enable: VkBool32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineViewportStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub viewport_count: u32,
    pub p_viewports: *const VkViewport,
    pub scissor_count: u32,
    pub p_scissors: *const VkRect2D,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub depth_clamp_enable: VkBool32,
    pub rasterizer_discard_enable: VkBool32,
    pub polygon_mode: VkPolygonMode,
    pub cull_mode: VkCullModeFlagBits,
    pub front_face: VkFrontFace,
    pub depth_bias_enable: VkBool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub patch_control_points: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlagBits,
    pub depth_test_enable: VkBool32,
    pub depth_write_enable: VkBool32,
    pub depth_compare_op: VkCompareOp,
    pub depth_bounds_test_enable: VkBool32,
    pub stencil_test_enable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub rasterization_samples: VkSampleCountFlagBits,
    pub sample_shading_enable: VkBool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const VkSampleMask,
    pub alpha_to_coverage_enable: VkBool32,
    pub alpha_to_one_enable: VkBool32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlagBits,
    pub logic_op_enable: VkBool32,
    pub logic_op: VkLogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const VkPipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blend_enable: VkBool32,
    pub src_color_blend_factor: VkBlendFactor,
    pub dst_color_blend_factor: VkBlendFactor,
    pub color_blend_op: VkBlendOp,
    pub src_alpha_blend_factor: VkBlendFactor,
    pub dst_alpha_blend_factor: VkBlendFactor,
    pub alpha_blend_op: VkBlendOp,
    pub color_write_mask: VkColorComponentFlagBits,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const VkDynamicState,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkGraphicsPipelineCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkPipelineCreateFlagBits,
    pub stage_count: u32,
    pub p_stages: *const VkPipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const VkPipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const VkPipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const VkPipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub render_pass: VkRenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: VkPipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VkVertexInputRate,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPushConstantRange {
    pub stage_flags: VkShaderStageFlagBits,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const VkSpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSpecializationMapEntry {
    pub constant_i_d: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkStencilOpState {
    pub fail_op: VkStencilOp,
    pub pass_op: VkStencilOp,
    pub depth_fail_op: VkStencilOp,
    pub compare_op: VkCompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

