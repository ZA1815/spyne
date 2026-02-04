use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{attachment_load_op::VkAttachmentLoadOp, attachment_store_op::VkAttachmentStoreOp, format::VkFormat, image_layout::VkImageLayout, pipeline_bind_point::VkPipelineBindPoint, structure_type::VkStructureType}, flags::{access::VkAccessFlagBits, attachment_description::VkAttachmentDescriptionFlagBits, dependency::VkDependencyFlagBits, framebuffer_create::VkFramebufferCreateFlagBits, pipeline_stage::VkPipelineStageFlagBits, render_pass_create::VkRenderPassCreateFlagBits, sample_count::VkSampleCountFlagBits, subpass_description::VkSubpassDescriptionFlagBits}}, types::image::VkImageView};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkRenderPass(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkFramebuffer(pub *mut c_void);

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkRenderPassCreateFlagBits,
    pub attachment_count: u32,
    pub p_attachments: *const VkAttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const VkSubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const VkSubpassDependency,
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkFramebufferCreateFlagBits,
    pub render_pass: VkRenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlagBits,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub load_op: VkAttachmentLoadOp,
    pub store_op: VkAttachmentStoreOp,
    pub stencil_load_op: VkAttachmentLoadOp,
    pub stencil_store_op: VkAttachmentStoreOp,
    pub initial_layout: VkImageLayout,
    pub final_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[repr(C)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlagBits,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const VkAttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const VkAttachmentReference,
    pub p_resolve_attachments: *const VkAttachmentReference,
    pub p_depth_stencil_attachment: *const VkAttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

#[repr(C)]
pub struct VkSubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: VkPipelineStageFlagBits,
    pub dst_stage_mask: VkPipelineStageFlagBits,
    pub src_access_mask: VkAccessFlagBits,
    pub dst_access_mask: VkAccessFlagBits,
    pub dependency_flags: VkDependencyFlagBits,
}

