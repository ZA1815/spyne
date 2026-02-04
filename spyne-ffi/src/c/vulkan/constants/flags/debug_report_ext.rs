#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDebugReportFlagBitsEXT(pub u32);

impl std::ops::BitOr for VkDebugReportFlagBitsEXT {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 0);
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 1);
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 2);
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 3);
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagBitsEXT = VkDebugReportFlagBitsEXT(1 << 4);
