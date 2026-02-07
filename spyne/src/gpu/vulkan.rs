use std::{ffi::{CStr, CString}, mem::MaybeUninit, ptr::{null, null_mut}};

use spyne_ffi::c::vulkan::{constants::{enums::{result::{VK_ERROR_DEVICE_LOST, VK_ERROR_EXTENSION_NOT_PRESENT, VK_ERROR_FEATURE_NOT_PRESENT, VK_ERROR_FORMAT_NOT_SUPPORTED, VK_ERROR_FRAGMENTED_POOL, VK_ERROR_INCOMPATIBLE_DRIVER, VK_ERROR_INITIALIZATION_FAILED, VK_ERROR_LAYER_NOT_PRESENT, VK_ERROR_MEMORY_MAP_FAILED, VK_ERROR_OUT_OF_DEVICE_MEMORY, VK_ERROR_OUT_OF_HOST_MEMORY, VK_ERROR_TOO_MANY_OBJECTS, VK_ERROR_UNKNOWN, VK_EVENT_RESET, VK_EVENT_SET, VK_INCOMPLETE, VK_NOT_READY, VK_SUCCESS, VK_TIMEOUT, VkResult}, structure_type::{VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO, VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO}}, flags::{device_queue_create::VkDeviceQueueCreateFlagBits, instance_create::VkInstanceCreateFlagBits, memory_property::{VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT, VkMemoryPropertyFlagBits}, queue::{VK_QUEUE_COMPUTE_BIT, VK_QUEUE_GRAPHICS_BIT, VkQueueFlagBits}, sample_count::{VK_SAMPLE_COUNT_1_BIT, VK_SAMPLE_COUNT_2_BIT, VK_SAMPLE_COUNT_4_BIT, VK_SAMPLE_COUNT_8_BIT, VK_SAMPLE_COUNT_16_BIT, VK_SAMPLE_COUNT_32_BIT, VK_SAMPLE_COUNT_64_BIT}}, versions::VK_API_VERSION_1_4}, functions::{DeviceFunctions, EntryFunctions, InstanceFunctions, PhysicalDeviceFunctions}, types::{device::{VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateInfo}, instance::{VkApplicationInfo, VkInstance, VkInstanceCreateInfo}, physical_device::{VkPhysicalDevice, VkPhysicalDeviceMemoryProperties, VkPhysicalDeviceProperties, VkQueueFamilyProperties}, queue::VkQueue}};

use crate::gpu::{Gpu, GpuError, MsaaSampleCount, QueueCapabilities, QueueRequest};

pub struct VulkanBackend {
    entry_functions: EntryFunctions,
    vk_instance: VkInstance,
    instance_functions: InstanceFunctions
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
        let instance_functions = unsafe { InstanceFunctions::load(entry_functions.vk_get_instance_proc_addr, vk_instance) };
        
        Ok(Self {
            entry_functions,
            vk_instance,
            instance_functions
        })
    }
}

impl Gpu for VulkanBackend {
    type PhysicalDevice = VulkanPhysicalDevice;
    type Device = VulkanDevice;
    type CommandQueue = VulkanQueue;
    
    fn enumerate_devices(&self) -> Result<Vec<Self::PhysicalDevice>, GpuError> {
        let mut num_devices: u32 = 0;
        let res = unsafe { (self.instance_functions.vk_enumerate_physical_devices)(self.vk_instance, &mut num_devices, null_mut()) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res))
        }
        let mut vk_phys_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(num_devices as usize);
        unsafe { vk_phys_devices.set_len(num_devices as usize); }
        let res = unsafe { (self.instance_functions.vk_enumerate_physical_devices)(self.vk_instance, &mut num_devices, vk_phys_devices.as_mut_ptr()) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res))
        }
        let phys_devices: Vec<VulkanPhysicalDevice> = vk_phys_devices
            .into_iter()
            .map(|vk_physical_device| {
                let physical_device_functions = unsafe {
                   PhysicalDeviceFunctions::load(self.entry_functions.vk_get_instance_proc_addr, self.vk_instance)
                };
                let mut vk_physical_device_properties = MaybeUninit::<VkPhysicalDeviceProperties>::uninit();
                let vk_physical_device_properties = unsafe {
                    (physical_device_functions.vk_get_physical_device_properties)(vk_physical_device, vk_physical_device_properties.as_mut_ptr());
                    vk_physical_device_properties.assume_init()
                };
                let mut vk_physical_device_memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::uninit();
                let vk_physical_device_memory_properties = unsafe {
                    (physical_device_functions.vk_get_physical_device_memory_properties)(vk_physical_device, vk_physical_device_memory_properties.as_mut_ptr());
                    vk_physical_device_memory_properties.assume_init()
                };
                let mut num_queue_family: u32 = 0;
                unsafe { (physical_device_functions.vk_get_physical_device_queue_family_properties)(vk_physical_device, &mut num_queue_family, null_mut()) };
                let mut queue_family_properties: Vec<VkQueueFamilyProperties> = Vec::with_capacity(num_queue_family as usize);
                unsafe { queue_family_properties.set_len(num_queue_family as usize); }
                unsafe { (physical_device_functions.vk_get_physical_device_queue_family_properties)(vk_physical_device, &mut num_queue_family, queue_family_properties.as_mut_ptr()) };
                VulkanPhysicalDevice {
                    vk_physical_device,
                    physical_device_functions,
                    vk_physical_device_properties,
                    queue_family_properties,
                    vk_physical_device_memory_properties
                }
            })
            .collect();
        
        Ok(phys_devices)
    }
    
    fn open_device(&self, physical_device: &Self::PhysicalDevice, queues: &[QueueRequest]) -> Result<Self::Device, GpuError> {
        let queue_num = queues.iter().map(|n| n.count).sum();
        let mut queue_indices: Vec<(usize, QueueCapabilities, usize)> = Vec::with_capacity(queue_num);
        let mut queue_family_properties = physical_device.queue_family_properties.to_vec();
        
        let mut p_queue_create_infos: Vec<VkDeviceQueueCreateInfo> = Vec::new();
        let mut p_queue_priorities_vec_store: Vec<Vec<f32>> = Vec::new();
        for q in queues {
            let p_queue_priorities_vec: Vec<f32> = vec![1.0; q.count];
            p_queue_priorities_vec_store.push(p_queue_priorities_vec);
            let vec_ptr = p_queue_priorities_vec_store.last().unwrap();
            let found = queue_family_properties.iter_mut().enumerate().filter(|(_, qf)| (q.capabilities.0 & qf.queue_flags.0) == q.capabilities.0).min_by_key(|(_, qf)| qf.queue_flags.0.count_ones());
            match found {
                Some(qf) => {
                    if q.count > qf.1.queue_count as usize {
                        return Err(GpuError::QueueCapabilityMismatch(format!("VulkanBackend: Requested too many queues for this family ({} > {})", q.count, qf.1.queue_count)))
                    }
                    let queue_info = VkDeviceQueueCreateInfo {
                        s_type: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                        p_next: null(),
                        flags: VkDeviceQueueCreateFlagBits(0),
                        queue_family_index: qf.0 as u32,
                        queue_count: q.count as u32,
                        p_queue_priorities: vec_ptr.as_ptr()
                    };
                    p_queue_create_infos.push(queue_info);
                    queue_indices.push((qf.0, q.capabilities, q.count));
                    qf.1.queue_count -= q.count as u32;
                },
                None => return Err(GpuError::QueueCapabilityMismatch(format!("VulkanBackend: Couldn't find a queue that supports {:#?}", q.capabilities)))
            }
        }
        
        let extension_name = CString::new("VK_KHR_swapchain").unwrap();
        let extension_ptr = extension_name.as_ptr();
        let device_info = VkDeviceCreateInfo {
            s_type: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            p_next: null(),
            flags: 0,
            queue_create_info_count: queues.len() as u32,
            p_queue_create_infos: p_queue_create_infos.as_ptr(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: null(),
            enabled_extension_count: 1,
            pp_enabled_extension_names: &extension_ptr,
            p_enabled_features: null()
        };
        
        let mut vk_device = VkDevice(null_mut());
        let res = unsafe { (self.instance_functions.vk_create_device)(physical_device.vk_physical_device, &device_info, null(), &mut vk_device) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res));
        }
        let device_functions = unsafe { DeviceFunctions::load(self.instance_functions.vk_get_device_proc_addr, vk_device) };
        let mut queues: Vec<VulkanQueue> = Vec::with_capacity(queue_num);
        for (fam_idx, fam_capabilities, num_queues) in queue_indices {
            for i in 0..num_queues {
                let mut vk_queue = VkQueue(null_mut());
                unsafe { (device_functions.vk_get_device_queue)(vk_device, fam_idx as u32, i as u32, &mut vk_queue) };
                queues.push(VulkanQueue { vk_queue, capabilities: fam_capabilities});
            }
        }
        
        Ok(VulkanDevice {
            vk_device,
            device_functions,
            queues
        })
    }
    
    fn device_name(&self, physical_device: &Self::PhysicalDevice) -> String {
        unsafe { CStr::from_ptr(physical_device.vk_physical_device_properties.device_name.as_ptr()).to_string_lossy().into_owned() }
    }
    
    fn supports_compute(&self, physical_device: &Self::PhysicalDevice) -> bool {
        physical_device
            .queue_family_properties
            .iter()
            .any(|queue_fam| queue_fam.queue_flags | VK_QUEUE_COMPUTE_BIT != VkQueueFlagBits(0))
    }
    
    fn has_unified_memory(&self, physical_device: &Self::PhysicalDevice) -> bool {
        let unified_bits = VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT | VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
        physical_device
            .vk_physical_device_memory_properties
            .memory_types
            .iter()
            .any(|mem_type| mem_type.property_flags & unified_bits == unified_bits)
    }
    
    fn max_texture_size_1d(&self, physical_device: &Self::PhysicalDevice) -> usize {
        physical_device.vk_physical_device_properties.limits.max_image_dimension_1d as usize
    }
    
    fn max_texture_size_2d(&self, physical_device: &Self::PhysicalDevice) -> usize {
        physical_device.vk_physical_device_properties.limits.max_image_dimension_2d as usize
    }
    
    fn max_texture_size_3d(&self, physical_device: &Self::PhysicalDevice) -> usize {
        physical_device.vk_physical_device_properties.limits.max_image_dimension_3d as usize
    }
    
    fn max_texture_size_cube(&self, physical_device: &Self::PhysicalDevice) -> usize {
        physical_device.vk_physical_device_properties.limits.max_image_dimension_cube as usize
    }
    
    fn supported_msaa_samples(&self, physical_device: &Self::PhysicalDevice) -> MsaaSampleCount {
        match physical_device.vk_physical_device_properties.limits.framebuffer_color_sample_counts {
            VK_SAMPLE_COUNT_1_BIT => MsaaSampleCount::ONE,
            VK_SAMPLE_COUNT_2_BIT => MsaaSampleCount::TWO,
            VK_SAMPLE_COUNT_4_BIT => MsaaSampleCount::FOUR,
            VK_SAMPLE_COUNT_8_BIT => MsaaSampleCount::EIGHT,
            VK_SAMPLE_COUNT_16_BIT => MsaaSampleCount::SIXTEEN,
            VK_SAMPLE_COUNT_32_BIT => MsaaSampleCount::THIRTY_TWO,
            VK_SAMPLE_COUNT_64_BIT => MsaaSampleCount::SIXTY_FOUR,
            _ => unreachable!()
        }
    }
}

pub struct VulkanPhysicalDevice {
    vk_physical_device: VkPhysicalDevice,
    physical_device_functions: PhysicalDeviceFunctions,
    vk_physical_device_properties: VkPhysicalDeviceProperties,
    queue_family_properties: Vec<VkQueueFamilyProperties>,
    vk_physical_device_memory_properties: VkPhysicalDeviceMemoryProperties
    
}

pub struct VulkanDevice {
    vk_device: VkDevice,
    device_functions: DeviceFunctions,
    queues: Vec<VulkanQueue>
}

pub struct VulkanQueue {
    vk_queue: VkQueue,
    capabilities: QueueCapabilities
}

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