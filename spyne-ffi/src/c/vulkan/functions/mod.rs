mod initialization;
use std::{ffi::CString, mem::transmute, ptr::null_mut};

pub use initialization::*;
use spyne_macros::VulkanFunctions;

use crate::c::{linux::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}}, vulkan::types::VkInstance};

pub struct VulkanFunctions {
    pub entry_functions: EntryFunctions
}

pub struct EntryFunctions {
    pub vk_get_instance_proc_addr: VkGetInstanceProcAddr,
    pub vk_create_instance: VkCreateInstance
}

impl EntryFunctions {
    pub unsafe fn load() -> Self {
        let lib = unsafe { dlopen(CString::new("libvulkan.so.1").unwrap().as_ptr(), RTLD_NOW) };
        let vk_get_instance_proc_addr: VkGetInstanceProcAddr = unsafe { transmute(dlsym(lib, CString::new("vkGetInstanceProcAddr").unwrap().as_ptr())) };
        let vk_create_instance: VkCreateInstance = unsafe { transmute(vk_get_instance_proc_addr(VkInstance(null_mut()), CString::new("vkCreateInstance").unwrap().as_ptr())) };
        Self {
            vk_create_instance,
            vk_get_instance_proc_addr
        }
    }
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkInstance, loader = VkGetInstanceProcAddr)]
pub struct InstanceFunctions {
    #[vulkan(name = "vkEnumeratePhysicalDevices")]
    vk_enumerate_physical_devices: VkEnumeratePhysicalDevices
}