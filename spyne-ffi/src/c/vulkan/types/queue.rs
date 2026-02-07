use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{result::VkResult, structure_type::VkStructureType}, flags::pipeline_stage::VkPipelineStageFlagBits}, types::{command_buffer::VkCommandBuffer, swapchain::VkSwapchainKHR, sync::VkSemaphore}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkQueue(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubmitInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const VkSemaphore,
    pub p_wait_dst_stage_mask: *const VkPipelineStageFlagBits,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const VkCommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPresentInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const VkSemaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const VkSwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut VkResult,
}

