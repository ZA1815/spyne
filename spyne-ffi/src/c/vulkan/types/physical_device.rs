use std::ffi::{c_char, c_void};

use crate::c::vulkan::constants::enums::physical_device_type::VkPhysicalDeviceType;




#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkPhysicalDevice(pub *mut c_void);

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_i_d: u32,
    pub device_i_d: u32,
    pub device_type: VkPhysicalDeviceType,
    pub device_name: c_char,
    pub pipeline_cache_u_u_i_d: u8,
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    pub robust_buffer_access: VkBool32,
    pub full_draw_index_uint32: VkBool32,
    pub image_cube_array: VkBool32,
    pub independent_blend: VkBool32,
    pub geometry_shader: VkBool32,
    pub tessellation_shader: VkBool32,
    pub sample_rate_shading: VkBool32,
    pub dual_src_blend: VkBool32,
    pub logic_op: VkBool32,
    pub multi_draw_indirect: VkBool32,
    pub draw_indirect_first_instance: VkBool32,
    pub depth_clamp: VkBool32,
    pub depth_bias_clamp: VkBool32,
    pub fill_mode_non_solid: VkBool32,
    pub depth_bounds: VkBool32,
    pub wide_lines: VkBool32,
    pub large_points: VkBool32,
    pub alpha_to_one: VkBool32,
    pub multi_viewport: VkBool32,
    pub sampler_anisotropy: VkBool32,
    pub texture_compression_e_t_c2: VkBool32,
    pub texture_compression_a_s_t_c__l_d_r: VkBool32,
    pub texture_compression_b_c: VkBool32,
    pub occlusion_query_precise: VkBool32,
    pub pipeline_statistics_query: VkBool32,
    pub vertex_pipeline_stores_and_atomics: VkBool32,
    pub fragment_stores_and_atomics: VkBool32,
    pub shader_tessellation_and_geometry_point_size: VkBool32,
    pub shader_image_gather_extended: VkBool32,
    pub shader_storage_image_extended_formats: VkBool32,
    pub shader_storage_image_multisample: VkBool32,
    pub shader_storage_image_read_without_format: VkBool32,
    pub shader_storage_image_write_without_format: VkBool32,
    pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
    pub shader_sampled_image_array_dynamic_indexing: VkBool32,
    pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
    pub shader_storage_image_array_dynamic_indexing: VkBool32,
    pub shader_clip_distance: VkBool32,
    pub shader_cull_distance: VkBool32,
    pub shader_float64: VkBool32,
    pub shader_int64: VkBool32,
    pub shader_int16: VkBool32,
    pub shader_resource_residency: VkBool32,
    pub shader_resource_min_lod: VkBool32,
    pub sparse_binding: VkBool32,
    pub sparse_residency_buffer: VkBool32,
    pub sparse_residency_image2_d: VkBool32,
    pub sparse_residency_image3_d: VkBool32,
    pub sparse_residency2_samples: VkBool32,
    pub sparse_residency4_samples: VkBool32,
    pub sparse_residency8_samples: VkBool32,
    pub sparse_residency16_samples: VkBool32,
    pub sparse_residency_aliased: VkBool32,
    pub variable_multisample_rate: VkBool32,
    pub inherited_queries: VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: VkMemoryType,
    pub memory_heap_count: u32,
    pub memory_heaps: VkMemoryHeap,
}

#[repr(C)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: VkQueueFlagBits,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D,
}

