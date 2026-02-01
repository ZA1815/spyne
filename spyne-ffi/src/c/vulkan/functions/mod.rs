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

#[cfg(test)]
mod test {
    use std::ptr::{null, null_mut};

    use crate::c::vulkan::{constants::{result::VK_SUCCESS, structure_type::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO}, functions::{EntryFunctions, InstanceFunctions}, types::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo, VkPhysicalDevice}};
    
    #[test]
    fn test_vulkan_funcs() {
        let entry_funcs = unsafe { EntryFunctions::load() };
        let mut instance = VkInstance(null_mut());
        let instance_info = VkInstanceCreateInfo {
            s_type: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: null(),
            flags: 0,
            p_application_info: null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: null()
        };
        let res_init = unsafe { (entry_funcs.vk_create_instance)(&instance_info, null(), &mut instance) };
        if res_init < 0 {
            panic!("Vulkan Failed.");
        }
        let instance_funcs = unsafe { InstanceFunctions::load(entry_funcs.vk_get_instance_proc_addr, instance) };
        let mut device_count: u32 = 0;
        let physical_device = VkPhysicalDevice(null_mut());
        let res_device = unsafe { (instance_funcs.vk_enumerate_physical_devices)(instance, &mut device_count, physical_device) };
        if res_device < 0 {
            panic!("Vulkan Failed.");
        }
        assert_eq!(res_init, VK_SUCCESS);
        assert!(device_count > 0);
    }
}