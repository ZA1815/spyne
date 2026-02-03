pub type VkCreateSwapchainKHR = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSwapchainCreateInfoKHR,
    p_create_info: *const VkSwapchainCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_swapchain: *mut VkSwapchainKHR,
) -> VkResult;

pub type VkDestroySwapchainKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut VkImage,
) -> VkResult;

pub type VkAcquireNextImageKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    p_image_index: *mut u32,
) -> VkResult;

