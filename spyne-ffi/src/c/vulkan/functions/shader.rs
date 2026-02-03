pub type VkCreateShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkShaderModuleCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_shader_module: *mut VkShaderModule,
) -> VkResult;

pub type VkDestroyShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    shader_module: VkShaderModule,
    p_allocator: *const VkAllocationCallbacks,
);

