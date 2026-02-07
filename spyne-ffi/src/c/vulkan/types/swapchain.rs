use std::ffi::c_void;

use crate::c::vulkan::{constants::{enums::{format::VkFormat, khr::{color_space::VkColorSpaceKHR, present_mode::VkPresentModeKHR}, sharing_mode::VkSharingMode, structure_type::VkStructureType}, flags::{image_usage::VkImageUsageFlagBits, khr::{composite_alpha::VkCompositeAlphaFlagBitsKHR, surface_transform::VkSurfaceTransformFlagBitsKHR, swapchain_create::VkSwapchainCreateFlagBitsKHR}}}, types::{base::VkBool32, image::VkExtent2D, surface::VkSurfaceKHR}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSwapchainKHR(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSwapchainCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkSwapchainCreateFlagBitsKHR,
    pub surface: VkSurfaceKHR,
    pub min_image_count: u32,
    pub image_format: VkFormat,
    pub image_color_space: VkColorSpaceKHR,
    pub image_extent: VkExtent2D,
    pub image_array_layers: u32,
    pub image_usage: VkImageUsageFlagBits,
    pub image_sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: VkSurfaceTransformFlagBitsKHR,
    pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
    pub present_mode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub old_swapchain: VkSwapchainKHR,
}

