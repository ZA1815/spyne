mod initialization;
use std::{ffi::c_void, mem::transmute};

pub use initialization::*;

use crate::c::linux::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}};

pub struct VulkanFunctions {
    vk_create_instance: VkCreateInstance,
    vk_enumerate_physical_devices: VkEnumeratePhysicalDevices,
    vk_get_instance_proc_addr: VkGetInstanceProcAddr
}

impl VulkanFunctions {
    pub unsafe fn load() -> Self {
        let lib_name = get_lib_name();
        let lib = load_lib(lib_name);
        
        Self {
            vk_create_instance: unsafe { transmute(dlsym(lib, "vkCreateInstance".as_ptr() as *const i8)) },
            vk_enumerate_physical_devices: unsafe { transmute(dlsym(lib, "vkEnumeratePhysicalDevices".as_ptr() as *const i8)) },
            vk_get_instance_proc_addr: unsafe { transmute(dlsym(lib, "vkGetInstanceProcAddr".as_ptr() as *const i8)) }
        }
    }
}

fn get_lib_name() -> &'static str {
    #[cfg(target_os = "linux")]
    return "libvulkan.so.1";
    
    #[cfg(target_os = "macos")]
    return "libvulkan.dylib";
    
    #[cfg(target_os = "windows")]
    return "vulkan-1.dll";
}

fn load_lib(lib_name: &str) -> *mut c_void {
    #[cfg(target_os = "linux")]
    unsafe { dlopen(lib_name.as_ptr() as *const i8, RTLD_NOW) }
    
    #[cfg(target_os = "macos")]
    
    #[cfg(target_os = "windows")]
}