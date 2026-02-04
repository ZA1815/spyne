use crate::c::vulkan::{constants::{enums::{pipeline_bind_point::VkPipelineBindPoint, result::VkResult, subpass_contents::VkSubpassContents}, flags::command_buffer_reset::VkCommandBufferResetFlagBits}, types::{base::VkDeviceSize, buffer::VkBuffer, command_buffer::{VkCommandBuffer, VkCommandBufferAllocateInfo, VkCommandBufferBeginInfo, VkCommandPool, VkCommandPoolCreateInfo, VkRenderPassBeginInfo}, device::VkDevice, instance::VkAllocationCallbacks, pipeline::{VkPipeline, VkRect2D, VkViewport}}};

pub type VkCreateCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkCommandPoolCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_command_pool: *mut VkCommandPool,
) -> VkResult;

pub type VkDestroyCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkAllocateCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkCommandBufferAllocateInfo,
    p_command_buffers: *mut VkCommandBuffer,
) -> VkResult;

pub type VkFreeCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const VkCommandBuffer,
);

pub type VkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_begin_info: *const VkCommandBufferBeginInfo,
) -> VkResult;

pub type VkEndCommandBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
) -> VkResult;

pub type VkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlagBits,
) -> VkResult;

pub type VkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_render_pass_begin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
);

pub type VkCmdEndRenderPass = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
);

pub type VkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_bind_point: VkPipelineBindPoint,
    pipeline: VkPipeline,
);

pub type VkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const VkBuffer,
    p_offsets: *const VkDeviceSize,
);

pub type VkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const VkViewport,
);

pub type VkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const VkRect2D,
);

pub type VkCmdDraw = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

