#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkResult(pub i32);

pub const VK_SUCCESS: VkResult = VkResult(0);
pub const VK_NOT_READY: VkResult = VkResult(1);
pub const VK_TIMEOUT: VkResult = VkResult(2);
pub const VK_EVENT_SET: VkResult = VkResult(3);
pub const VK_EVENT_RESET: VkResult = VkResult(4);
pub const VK_INCOMPLETE: VkResult = VkResult(5);
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult(-1);
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult(-2);
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = VkResult(-3);
pub const VK_ERROR_DEVICE_LOST: VkResult = VkResult(-4);
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = VkResult(-5);
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = VkResult(-6);
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult(-7);
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult(-8);
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult(-9);
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = VkResult(-10);
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult(-11);
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = VkResult(-12);
pub const VK_ERROR_UNKNOWN: VkResult = VkResult(-13);
