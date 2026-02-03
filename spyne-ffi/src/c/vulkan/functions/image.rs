pub type VkCreateImage = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkImageCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_image: *mut VkImage,
) -> VkResult;

pub type VkDestroyImage = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_memory_requirements: *mut VkMemoryRequirements,
);

pub type VkBindImageMemory = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

pub type VkCreateImageView = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkImageViewCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_view: *mut VkImageView,
) -> VkResult;

pub type VkDestroyImageView = unsafe extern "system" fn(
    device: VkDevice,
    image_view: VkImageView,
    p_allocator: *const VkAllocationCallbacks,
);

