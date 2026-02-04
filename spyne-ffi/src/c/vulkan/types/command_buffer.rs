use std::ffi::c_void;

use std::ffi::c_char;

use crate::c::vulkan::{constants::{enums::{command_buffer_level::VkCommandBufferLevel, structure_type::VkStructureType}, flags::{command_buffer_usage::VkCommandBufferUsageFlagBits, command_pool_create::VkCommandPoolCreateFlagBits, query_control::VkQueryControlFlagBits, query_pipeline_statistic::VkQueryPipelineStatisticFlagBits}}, types::{base::VkBool32, pipeline::VkRect2D, render_pass::{VkFramebuffer, VkRenderPass}}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkCommandPool(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkCommandBuffer(pub *mut c_void);

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkCommandPoolCreateFlagBits,
    pub queue_family_index: u32,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub command_pool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkCommandBufferUsageFlagBits,
    pub p_inheritance_info: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub render_pass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusion_query_enable: VkBool32,
    pub query_flags: VkQueryControlFlagBits,
    pub pipeline_statistics: VkQueryPipelineStatisticFlagBits,
}

#[repr(C)]
pub struct VkRenderPassBeginInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub render_pass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub render_area: VkRect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const VkClearValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depth_stencil: VkClearDepthStencilValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
}

