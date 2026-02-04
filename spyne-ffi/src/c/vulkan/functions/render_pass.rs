use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, render_pass::{VkFramebuffer, VkFramebufferCreateInfo, VkRenderPass, VkRenderPassCreateInfo}}};

pub type VkCreateRenderPass = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkRenderPassCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_render_pass: *mut VkRenderPass,
) -> VkResult;

pub type VkDestroyRenderPass = unsafe extern "system" fn(
    device: VkDevice,
    render_pass: VkRenderPass,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkCreateFramebuffer = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkFramebufferCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_framebuffer: *mut VkFramebuffer,
) -> VkResult;

pub type VkDestroyFramebuffer = unsafe extern "system" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    p_allocator: *const VkAllocationCallbacks,
);

