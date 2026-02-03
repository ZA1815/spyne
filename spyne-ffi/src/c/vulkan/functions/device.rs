pub type VkGetDeviceQueue = unsafe extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut VkQueue,
);

pub type VkDeviceWaitIdle = unsafe extern "system" fn(
    device: VkDevice,
    device: VkDevice,
) -> VkResult;

pub type VkDestroyDevice = unsafe extern "system" fn(
    device: VkDevice,
    p_allocator: *const VkAllocationCallbacks,
    p_allocator: *const VkAllocationCallbacks,
);

