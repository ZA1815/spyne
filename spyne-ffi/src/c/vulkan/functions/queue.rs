pub type VkQueueSubmit = unsafe extern "system" fn(
    queue: VkQueue,
    submit_count: u32,
    p_submits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult;

pub type VkQueueWaitIdle = unsafe extern "system" fn(
    queue: VkQueue,
) -> VkResult;

pub type VkQueuePresentKHR = unsafe extern "system" fn(
    queue: VkQueue,
    p_present_info: *const VkPresentInfoKHR,
) -> VkResult;

