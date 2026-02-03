use std::ffi::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSurfaceKHR(pub *mut c_void);

#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: VkExtent2D,
    pub min_image_extent: VkExtent2D,
    pub max_image_extent: VkExtent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: VkSurfaceTransformFlagsKHR,
    pub current_transform: VkSurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
    pub supported_usage_flags: VkImageUsageFlags,
}

#[repr(C)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub color_space: VkColorSpaceKHR,
}

#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagsKHR,
    pub display: *struct wl_display,
    pub surface: *struct wl_surface,
}

