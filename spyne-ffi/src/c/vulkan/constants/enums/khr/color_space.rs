#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkColorSpaceKHR(pub u32);

pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR(0);
