use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{format::VkFormat, khr::color_space::VkColorSpaceKHR, structure_type::VkStructureType}, flags::{image_usage::VkImageUsageFlagBits, khr::{composite_alpha::VkCompositeAlphaFlagBitsKHR, surface_transform::VkSurfaceTransformFlagBitsKHR}}}, types::{base::VkFlags, image::VkExtent2D}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSurfaceKHR(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: VkExtent2D,
    pub min_image_extent: VkExtent2D,
    pub max_image_extent: VkExtent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: VkSurfaceTransformFlagBitsKHR,
    pub current_transform: VkSurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: VkCompositeAlphaFlagBitsKHR,
    pub supported_usage_flags: VkImageUsageFlagBits,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub color_space: VkColorSpaceKHR,
}

#[cfg(target_os = "linux")]
pub use wayland_surface_create_info_khr::*;

#[cfg(target_os = "linux")]
mod wayland_surface_create_info_khr {
    use crate::c::linux::wayland::{wl_display, wl_surface};
    use super::*;

    #[repr(C)]
    pub struct VkWaylandSurfaceCreateInfoKHR {
        pub s_type: VkStructureType,
        pub p_next: *const c_void,
        // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
        pub flags: VkFlags,
        pub display: *mut wl_display,
        pub surface: *mut wl_surface,
    }
}

