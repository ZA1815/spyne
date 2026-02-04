use std::ffi::c_void;

use crate::c::vulkan::constants::{enums::structure_type::VkStructureType, flags::command_pool_create::VkCommandPoolCreateFlagBits};


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
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depth_stencil: VkClearDepthStencilValue,
}

#[repr(C)]
pub union VkClearColorValue {
    pub float32: f32,
    pub int32: i32,
    pub uint32: u32,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: f32,
}

