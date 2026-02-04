use crate::c::vulkan::{constants::enums::result::VkResult, types::{device::VkDevice, instance::VkAllocationCallbacks, pipeline::{VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineCache, VkPipelineLayout, VkPipelineLayoutCreateInfo}}};

pub type VkCreatePipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkPipelineLayoutCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipeline_layout: *mut VkPipelineLayout,
) -> VkResult;

pub type VkDestroyPipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_info_count: u32,
    p_create_infos: *const VkGraphicsPipelineCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipelines: *mut VkPipeline,
) -> VkResult;

pub type VkDestroyPipeline = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    p_allocator: *const VkAllocationCallbacks,
);

