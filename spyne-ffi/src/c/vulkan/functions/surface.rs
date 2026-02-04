use crate::c::vulkan::{constants::enums::{khr::present_mode::VkPresentModeKHR, result::VkResult}, types::{base::VkBool32, instance::{VkAllocationCallbacks, VkInstance}, physical_device::VkPhysicalDevice, surface::{VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR}}};

pub type VkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: *mut VkBool32,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut VkPresentModeKHR,
) -> VkResult;

#[cfg(target_os = "linux")]
pub use khr_create_wayland_surface::*;

#[cfg(target_os = "linux")]
mod khr_create_wayland_surface {
   use crate::c::vulkan::types::surface::VkWaylandSurfaceCreateInfoKHR;
   use super::*;

    pub type VkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
       instance: VkInstance,
       p_create_info: *const VkWaylandSurfaceCreateInfoKHR,
       p_allocator: *const VkAllocationCallbacks,
       p_surface: *mut VkSurfaceKHR,
    ) -> VkResult;
}

