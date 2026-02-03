pub type VkCreateFence = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkFenceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_fence: *mut VkFence,
) -> VkResult;

pub type VkDestroyFence = unsafe extern "system" fn(
    device: VkDevice,
    fence: VkFence,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkWaitForFences = unsafe extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    p_fences: *const VkFence,
    wait_all: VkBool32,
    timeout: u64,
) -> VkResult;

pub type VkResetFences = unsafe extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    p_fences: *const VkFence,
) -> VkResult;

pub type VkCreateSemaphore = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSemaphoreCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_semaphore: *mut VkSemaphore,
) -> VkResult;

pub type VkDestroySemaphore = unsafe extern "system" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    p_allocator: *const VkAllocationCallbacks,
);

