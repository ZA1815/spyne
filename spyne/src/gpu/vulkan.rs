use std::{ffi::CString, ptr::{null, null_mut}};

use spyne_ffi::c::vulkan::{constants::{enums::{result::{VK_ERROR_DEVICE_LOST, VK_ERROR_EXTENSION_NOT_PRESENT, VK_ERROR_FEATURE_NOT_PRESENT, VK_ERROR_FORMAT_NOT_SUPPORTED, VK_ERROR_FRAGMENTED_POOL, VK_ERROR_INCOMPATIBLE_DRIVER, VK_ERROR_INITIALIZATION_FAILED, VK_ERROR_LAYER_NOT_PRESENT, VK_ERROR_MEMORY_MAP_FAILED, VK_ERROR_OUT_OF_DEVICE_MEMORY, VK_ERROR_OUT_OF_HOST_MEMORY, VK_ERROR_TOO_MANY_OBJECTS, VK_ERROR_UNKNOWN, VK_EVENT_RESET, VK_EVENT_SET, VK_INCOMPLETE, VK_NOT_READY, VK_SUCCESS, VK_TIMEOUT, VkResult}, structure_type::{VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO}}, flags::instance_create::VkInstanceCreateFlagBits, versions::VK_API_VERSION_1_4}, functions::{EntryFunctions, InstanceFunctions}, types::{instance::{VkAllocationCallbacks, VkApplicationInfo, VkInstance, VkInstanceCreateInfo}, physical_device::VkPhysicalDevice}};

use crate::gpu::{Gpu, GpuError};

pub struct VulkanBackend {
    entry_functions: EntryFunctions,
    vk_instance: VkInstance
}

impl VulkanBackend {
    pub fn new(extensions: &[CString], debug: bool) -> Result<Self, GpuError> {
        let app_info = VkApplicationInfo {
            s_type: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            p_next: null(),
            p_application_name: null(),
            application_version: 0,
            p_engine_name: null(),
            engine_version: 0,
            api_version: VK_API_VERSION_1_4
        };
        
        let validation_str;
        let validation_ptr;
        let enabled_layer_count;
        let pp_enabled_layer_names: *const *const i8;
        
        if debug {
            validation_str = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
            validation_ptr = validation_str.as_ptr();
            enabled_layer_count = 1;
            pp_enabled_layer_names = &validation_ptr;
        }
        else {
            enabled_layer_count = 0;
            pp_enabled_layer_names = null::<*const i8>();
        };
        
        let mut enabled_extension_count: u32 = 0;
        let mut extension_names: Vec<CString>;
        let extension_ptrs: Vec<*const i8>;
        let pp_enabled_extension_names: *const *const i8;
        
        if debug {
            extension_names = vec![
                CString::new("VK_KHR_surface").unwrap(),
                CString::new("VK_KHR_wayland_surface").unwrap(),
                CString::new("VK_EXT_debug_utils").unwrap()
            ];
            extension_names.extend_from_slice(extensions);
            enabled_extension_count += extension_names.len() as u32;
        }
        else {
            extension_names = vec![
                CString::new("VK_KHR_surface").unwrap(),
                CString::new("VK_KHR_wayland_surface").unwrap(),
            ];
            extension_names.extend_from_slice(extensions);
            enabled_extension_count += extension_names.len() as u32;
        };
        
        extension_ptrs = extension_names.iter().map(|cs| cs.as_ptr()).collect();
        pp_enabled_extension_names = extension_ptrs.as_ptr();
        
        let instance_info = VkInstanceCreateInfo {
            s_type: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: null(),
            flags: VkInstanceCreateFlagBits(0),
            p_application_info: &app_info,
            enabled_layer_count,
            pp_enabled_layer_names,
            enabled_extension_count,
            pp_enabled_extension_names
        };
        
        let entry_functions = unsafe { EntryFunctions::load() };
        let mut vk_instance = VkInstance(null_mut());
        let res = unsafe { (entry_functions.vk_create_instance)(&instance_info, null(), &mut vk_instance) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res));
        }
        
        Ok(Self {
            entry_functions,
            vk_instance
        })
    }
}

impl Gpu for VulkanBackend {
    type PhysicalDevice = VulkanPhysicalDevice;
    fn enumerate_devices(&self) -> Result<Vec<Self::PhysicalDevice>, GpuError> {
        let instance_functions = unsafe { InstanceFunctions::load(self.entry_functions.vk_get_instance_proc_addr, self.vk_instance) };
        let mut num_devices: u32 = 0;
        let res = unsafe { (instance_functions.vk_enumerate_physical_devices)(self.vk_instance, &mut num_devices, null_mut()) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res))
        }
        let mut vk_phys_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(num_devices as usize);
        unsafe { vk_phys_devices.set_len(num_devices as usize); }
        let res = unsafe { (instance_functions.vk_enumerate_physical_devices)(self.vk_instance, &mut num_devices, vk_phys_devices.as_mut_ptr()) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res))
        }
        let phys_devices: Vec<VulkanPhysicalDevice> = vk_phys_devices
            .into_iter()
            .map(|pd| VulkanPhysicalDevice(pd))
            .collect();
        
        Ok(phys_devices)
    }
}

pub struct VulkanPhysicalDevice(pub VkPhysicalDevice);

pub enum VulkanExtensions {
    
}

fn map_vk_error(res: VkResult) -> GpuError {
    match res {
        VK_NOT_READY => GpuError::BackendError { code: VK_NOT_READY.0, message: "VulkanBackend: VK_NOT_READY".to_string() },
        VK_TIMEOUT => GpuError::BackendError { code: VK_TIMEOUT.0, message: "VulkanBackend: VK_TIMEOUT".to_string() },
        VK_EVENT_SET => GpuError::BackendError { code: VK_EVENT_SET.0, message: "VulkanBackend: VK_EVENT_SET".to_string() },
        VK_EVENT_RESET => GpuError::BackendError { code: VK_EVENT_RESET.0, message: "VulkanBackend: VK_EVENT_RESET".to_string() },
        VK_INCOMPLETE => GpuError::BackendError { code: VK_INCOMPLETE.0, message: "VulkanBackend: VK_INCOMPLETE".to_string() },
        VK_ERROR_OUT_OF_HOST_MEMORY => GpuError::OutOfMemory("VulkanBackend: VK_ERROR_OUT_OF_HOST_MEMORY".to_string()),
        VK_ERROR_OUT_OF_DEVICE_MEMORY => GpuError::OutOfMemory("VulkanBackend: VK_ERROR_OUT_OF_DEVICE_MEMORY".to_string()),
        VK_ERROR_INITIALIZATION_FAILED => GpuError::InitializationFailed("VulkanBackend: VK_ERROR_INITIALIZATION_FAILED".to_string()),
        VK_ERROR_DEVICE_LOST => GpuError::BackendError { code: VK_ERROR_DEVICE_LOST.0, message: "VulkanBackend: VK_ERROR_DEVICE_LOST".to_string() },
        VK_ERROR_MEMORY_MAP_FAILED => GpuError::BackendError { code: VK_ERROR_MEMORY_MAP_FAILED.0, message: "VulkanBackend: VK_ERROR_MEMORY_MAP_FAILED".to_string() },
        VK_ERROR_LAYER_NOT_PRESENT => GpuError::BackendError { code: VK_ERROR_LAYER_NOT_PRESENT.0, message: "VulkanBackend: VK_ERROR_LAYER_NOT_PRESENT".to_string() },
        VK_ERROR_EXTENSION_NOT_PRESENT => GpuError::BackendError { code: VK_ERROR_EXTENSION_NOT_PRESENT.0, message: "VulkanBackend: VK_ERROR_EXTENSION_NOT_PRESENT".to_string() },
        VK_ERROR_FEATURE_NOT_PRESENT => GpuError::FeatureNotSupported("VulkanBackend: VK_ERROR_FEATURE_NOT_PRESENT".to_string()),
        VK_ERROR_INCOMPATIBLE_DRIVER => GpuError::BackendError { code: VK_ERROR_INCOMPATIBLE_DRIVER.0, message: "VulkanBackend: VK_ERROR_INCOMPATIBLE_DRIVER".to_string() },
        VK_ERROR_TOO_MANY_OBJECTS => GpuError::BackendError { code: VK_ERROR_TOO_MANY_OBJECTS.0, message: "VulkanBackend: VK_ERROR_TOO_MANY_OBJECTS".to_string() },
        VK_ERROR_FORMAT_NOT_SUPPORTED => GpuError::BackendError { code: VK_ERROR_FORMAT_NOT_SUPPORTED.0, message: "VulkanBackend: VK_ERROR_FORMAT_NOT_SUPPORTED".to_string() },
        VK_ERROR_FRAGMENTED_POOL => GpuError::BackendError { code: VK_ERROR_FRAGMENTED_POOL.0, message: "VulkanBackend: VK_ERROR_FRAGMENTED_POOL".to_string() },
        VK_ERROR_UNKNOWN => GpuError::BackendError { code: VK_ERROR_UNKNOWN.0, message: "VulkanBackend: Unknown error".to_string() },
        _ => GpuError::BackendError { code: -14, message: "VulkanBackend: Unknown error (error variant unkown)".to_string() }
    }
}