pub type VkAllocateMemory = unsafe extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkMemoryAllocateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_memory: *mut VkDeviceMemory,
) -> VkResult;

pub type VkFreeMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkMapMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> VkResult;

pub type VkUnmapMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
);

