#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceMemoryReportEventTypeEXT(pub u32);

pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(0);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(1);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(2);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(3);
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: VkDeviceMemoryReportEventTypeEXT = VkDeviceMemoryReportEventTypeEXT(4);
