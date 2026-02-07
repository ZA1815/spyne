use std::{ffi::{CStr, CString, c_void}, mem::MaybeUninit, ptr::{copy_nonoverlapping, null, null_mut}};

use spyne_ffi::c::vulkan::{constants::{enums::{format::{VK_FORMAT_B8G8R8A8_UNORM, VK_FORMAT_D24_UNORM_S8_UINT, VK_FORMAT_D32_SFLOAT, VK_FORMAT_R8_UNORM, VK_FORMAT_R8G8B8A8_UNORM, VK_FORMAT_R16G16B16A16_SFLOAT, VK_FORMAT_R32G32B32A32_SFLOAT}, image_layout::VK_IMAGE_LAYOUT_UNDEFINED, image_tiling::VK_IMAGE_TILING_OPTIMAL, image_type::{VK_IMAGE_TYPE_1D, VK_IMAGE_TYPE_2D, VK_IMAGE_TYPE_3D}, result::{VK_ERROR_DEVICE_LOST, VK_ERROR_EXTENSION_NOT_PRESENT, VK_ERROR_FEATURE_NOT_PRESENT, VK_ERROR_FORMAT_NOT_SUPPORTED, VK_ERROR_FRAGMENTED_POOL, VK_ERROR_INCOMPATIBLE_DRIVER, VK_ERROR_INITIALIZATION_FAILED, VK_ERROR_LAYER_NOT_PRESENT, VK_ERROR_MEMORY_MAP_FAILED, VK_ERROR_OUT_OF_DEVICE_MEMORY, VK_ERROR_OUT_OF_HOST_MEMORY, VK_ERROR_TOO_MANY_OBJECTS, VK_ERROR_UNKNOWN, VK_EVENT_RESET, VK_EVENT_SET, VK_INCOMPLETE, VK_NOT_READY, VK_SUCCESS, VK_TIMEOUT, VkResult}, sharing_mode::VK_SHARING_MODE_EXCLUSIVE, structure_type::{VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO, VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO, VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO}}, flags::{buffer_create::VkBufferCreateFlagBits, buffer_usage::VkBufferUsageFlagBits, device_queue_create::VkDeviceQueueCreateFlagBits, image_create::{VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT, VkImageCreateFlagBits}, image_usage::{VK_IMAGE_USAGE_SAMPLED_BIT, VkImageUsageFlagBits}, instance_create::VkInstanceCreateFlagBits, memory_map::VkMemoryMapFlagBits, memory_property::{VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, VK_MEMORY_PROPERTY_HOST_COHERENT_BIT, VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT}, queue::{VK_QUEUE_COMPUTE_BIT, VkQueueFlagBits}, sample_count::{VK_SAMPLE_COUNT_1_BIT, VK_SAMPLE_COUNT_2_BIT, VK_SAMPLE_COUNT_4_BIT, VK_SAMPLE_COUNT_8_BIT, VK_SAMPLE_COUNT_16_BIT, VK_SAMPLE_COUNT_32_BIT, VK_SAMPLE_COUNT_64_BIT, VkSampleCountFlagBits}}, versions::VK_API_VERSION_1_4}, functions::{DeviceFunctions, EntryFunctions, InstanceFunctions, PhysicalDeviceFunctions, VulkanFunctions}, types::{buffer::{VkBuffer, VkBufferCreateInfo}, device::{VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateInfo}, image::{VkExtent3D, VkImage, VkImageCreateInfo}, instance::{VkApplicationInfo, VkInstance, VkInstanceCreateInfo}, memory::{VkDeviceMemory, VkMemoryAllocateInfo, VkMemoryRequirements}, physical_device::{VkPhysicalDevice, VkPhysicalDeviceMemoryProperties, VkPhysicalDeviceProperties, VkQueueFamilyProperties}, queue::VkQueue}};

use crate::graphics::{BufferUsage, Graphics, GraphicsError, MemoryLocation, MsaaSampleCount, QueueCapabilities, QueueRequest, TextureDimension, TextureFormat, TextureUsage}, TextureFormat

pub struct VulkanBackend {
    entry_functions: EntryFunctions,
    vk_instance: VkInstance,
    instance_functions: InstanceFunctions
}

impl VulkanBackend {
    pub fn new(extensions: &[CString], debug: bool) -> Result<Self, GraphicsError> {
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

impl Graphics for VulkanBackend {
    type PhysicalDevice = VulkanPhysicalDevice;
    type Device<'a> = VulkanDevice<'a>;
    type CommandQueue = VulkanQueue;
    type Buffer = VulkanBuffer;
    type Texture = VulkanImage;
    
    fn enumerate_devices(&self) -> Result<Vec<Self::PhysicalDevice>, GraphicsError> {
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
                    functions: physical_device_functions,
                    vk_physical_device_properties,
                    queue_family_properties,
                    vk_physical_device_memory_properties
                }
            })
            .collect();
        
        Ok(phys_devices)
    }
    
    fn open_device<'a>(&self, physical_device: &'a Self::PhysicalDevice, queues: &[QueueRequest]) -> Result<Self::Device<'a>, GraphicsError> {
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
                        return Err(GraphicsError::QueueCapabilityMismatch(format!("VulkanBackend: Requested too many queues for this family ({} > {})", q.count, qf.1.queue_count)))
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
                None => return Err(GraphicsError::QueueCapabilityMismatch(format!("VulkanBackend: Couldn't find a queue that supports {:#?}", q.capabilities)))
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
        
        let functions = unsafe { VulkanFunctions::load(self.entry_functions, self.vk_instance, vk_device) };
        
        Ok(VulkanDevice {
            vk_device,
            physical_device,
            functions,
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
    
    fn create_buffer<'a>(&self, device: &Self::Device<'a>, size: usize, usage: BufferUsage, location: MemoryLocation) -> Result<Self::Buffer, GraphicsError> {
        let buffer_create_info = VkBufferCreateInfo {
            s_type: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            p_next: null(),
            flags: VkBufferCreateFlagBits(0),
            size: size as u64,
            usage: VkBufferUsageFlagBits(usage.0),
            sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: null(),
        };
        let mut vk_buffer = VkBuffer(null_mut());
        let res = unsafe { (device.functions.buffer_functions.vk_create_buffer)(device.vk_device, &buffer_create_info, null(), &mut vk_buffer) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res));
        }
        let mut vk_memory_requirements = MaybeUninit::<VkMemoryRequirements>::uninit();
        let vk_memory_requirements = unsafe {
            (device.functions.buffer_functions.vk_get_buffer_memory_requirements)(device.vk_device, vk_buffer, vk_memory_requirements.as_mut_ptr());
            vk_memory_requirements.assume_init()
        };
        
        let mem_type = device
            .physical_device
            .vk_physical_device_memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(idx, mem_type)| {
                ((1 << idx) & vk_memory_requirements.memory_type_bits != 0)
                &&
                {
                    let shared_bits = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
                    let private_bits = VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT;
                    match location {
                        MemoryLocation::Shared => mem_type.property_flags.0 & shared_bits.0 == shared_bits.0,
                        // Handle staging buffer in this case
                        MemoryLocation::Private => mem_type.property_flags.0 & private_bits.0 == private_bits.0
                    }
                }
            });
        let memory_allocate_info: VkMemoryAllocateInfo = match mem_type {
            Some(mt) => {
                VkMemoryAllocateInfo {
                    s_type: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                    p_next: null(),
                    allocation_size: vk_memory_requirements.size,
                    memory_type_index: mt.0 as u32
                }
            }
            // Likely change this error type when I add more GraphicsError variants
            None => return Err(GraphicsError::FeatureNotSupported("VulkanBackend: Couldn't find a supported memory type that fits requirements".to_string()))
        };
        let mut vk_device_memory = VkDeviceMemory(null_mut());
        let res = unsafe { (device.functions.memory_functions.vk_allocate_memory)(device.vk_device, &memory_allocate_info, null(), &mut vk_device_memory) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res));
        }
        let res = unsafe { (device.functions.buffer_functions.vk_bind_buffer_memory)(device.vk_device, vk_buffer, vk_device_memory, 0) };
        if res != VK_SUCCESS {
            return Err(map_vk_error(res));
        }
        
        Ok(VulkanBuffer {
            vk_buffer,
            vk_device_memory,
            allocated_size: vk_memory_requirements.size as usize,
            location
        })
    }
    
    fn read_buffer<'a>(&self, device: &Self::Device<'a>, buffer: &Self::Buffer, offset: usize, size: usize) -> Result<Vec<u8>, GraphicsError> {
        match buffer.location {
            MemoryLocation::Shared => {
                let mut pp_data = MaybeUninit::<*mut c_void>::uninit();
                let res = unsafe { (device.functions.memory_functions.vk_map_memory)(device.vk_device, buffer.vk_device_memory, offset as u64, size as u64, VkMemoryMapFlagBits(0), pp_data.as_mut_ptr()) };
                if res != VK_SUCCESS {
                    return Err(map_vk_error(res))
                }
                let pp_data = unsafe { pp_data.assume_init() };
                let mut buf_read: Vec<u8> = Vec::with_capacity(size);
                unsafe { buf_read.set_len(size) };
                unsafe { copy_nonoverlapping(pp_data, buf_read.as_mut_ptr() as *mut c_void, size) };
                unsafe { (device.functions.memory_functions.vk_unmap_memory)(device.vk_device, buffer.vk_device_memory) }
                
                Ok(buf_read)
            }
            // Likely change this error type when I add more GraphicsError variants
            MemoryLocation::Private => Err(GraphicsError::FeatureNotSupported("VulkanBackend: Private buffers are not allowed to be read from.".to_string()))
        }
    }
    
    fn write_buffer<'a>(&self, device: &Self::Device<'a>, buffer: &Self::Buffer, offset: usize, data: &[u8]) -> Result<(), GraphicsError> {
        match buffer.location {
            MemoryLocation::Shared => {
                let mut pp_data = MaybeUninit::<*mut c_void>::uninit();
                let res = unsafe { (device.functions.memory_functions.vk_map_memory)(device.vk_device, buffer.vk_device_memory, offset as u64, data.len() as u64, VkMemoryMapFlagBits(0), pp_data.as_mut_ptr()) };
                if res != VK_SUCCESS {
                    return Err(map_vk_error(res));
                }
                let pp_data = unsafe { pp_data.assume_init() };
                unsafe { copy_nonoverlapping(data.as_ptr(), pp_data as *mut u8, data.len()) };
                unsafe { (device.functions.memory_functions.vk_unmap_memory)(device.vk_device, buffer.vk_device_memory) };
                
                Ok(())
            }
            // Likely change this error type when I add more GraphicsError variants
            MemoryLocation::Private => Err(GraphicsError::FeatureNotSupported("VulkanBackend: Private buffers are not allowed to be wrote to.".to_string()))
        }
    }
    
    fn create_texture<'a>(
            &self,
            device: &Self::Device<'a>,
            dimension: TextureDimension,
            format: TextureFormat,
            width: u32,
            height: u32,
            depth: u32,
            layers: u32,
            mip_levels: u32,
            samples: MsaaSampleCount,
            usage: TextureUsage
        ) -> Result<Self::Texture, GpuError> {
            let flags = match dimension {
                TextureDimension::Cube => VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT,
                _ => VkImageCreateFlagBits(0)
            };
            let image_type = match dimension {
                TextureDimension::D1 => VK_IMAGE_TYPE_1D,
                TextureDimension::D2 => VK_IMAGE_TYPE_2D,
                TextureDimension::D3 => VK_IMAGE_TYPE_3D,
                TextureDimension::Cube => VK_IMAGE_TYPE_2D
            };
            let image_format = match format {
                TextureFormat::RGBA8 => VK_FORMAT_R8G8B8A8_UNORM,
                TextureFormat::BGRA8 => VK_FORMAT_B8G8R8A8_UNORM,
                TextureFormat::R8 => VK_FORMAT_R8_UNORM,
                TextureFormat::RGBA16F => VK_FORMAT_R16G16B16A16_SFLOAT,
                TextureFormat::RGBA32F => VK_FORMAT_R32G32B32A32_SFLOAT,
                TextureFormat::Depth24 => VK_FORMAT_D24_UNORM_S8_UINT,
                TextureFormat::Depth32F => VK_FORMAT_D32_SFLOAT
            };
            let extent = VkExtent3D { width, height, depth };
            let array_layers = match dimension {
                TextureDimension::Cube => layers * 6,
                _ => layers
            };
            let samples = VkSampleCountFlagBits(samples.0);
            let image_create_info = VkImageCreateInfo {
                s_type: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                p_next: null(),
                flags,
                image_type,
                format: image_format,
                extent,
                mip_levels,
                array_layers,
                samples,
                tiling: VK_IMAGE_TILING_OPTIMAL,
                usage: VkImageUsageFlagBits(usage.0),
                sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: null(),
                initial_layout: VK_IMAGE_LAYOUT_UNDEFINED
            };
            let mut vk_image = VkImage(null_mut());
            let res = unsafe { (device.functions.image_functions.vk_create_image)(device.vk_device, &image_create_info, null(), &mut vk_image) };
            if res != VK_SUCCESS {
                return Err(map_vk_error(res));
            }
            
            let mut vk_memory_requirements = MaybeUninit::<VkMemoryRequirements>::uninit();
            let vk_memory_requirements = unsafe {
                (device.functions.image_functions.vk_get_image_memory_requirements)(device.vk_device, vk_image, vk_memory_requirements.as_mut_ptr());
                vk_memory_requirements.assume_init()
            };
            let mem_type = device
                .physical_device
                .vk_physical_device_memory_properties
                .memory_types
                .iter()
                .enumerate()
                .find(|(idx, mem_type)| {
                    (1 << idx) & vk_memory_requirements.memory_type_bits != 0
                    &&
                    mem_type.property_flags.0 & VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0 == VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0
                });
            let memory_allocate_info: VkMemoryAllocateInfo = match mem_type {
                Some(mt) => {
                    VkMemoryAllocateInfo {
                        s_type: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                        p_next: null(),
                        allocation_size: vk_memory_requirements.size,
                        memory_type_index: mt.0 as u32
                    }
                }
                None => return Err(GraphicsError::QueueCapabilityMismatch(format!("VulkanBackend: Couldn't find a queue that supports {:#?}", q.capabilities)))
            };
            let mut vk_device_memory = VkDeviceMemory(null_mut());
            let res = unsafe { (device.functions.memory_functions.vk_allocate_memory)(device.vk_device, &memory_allocate_info, null(), &mut vk_device_memory) };
            if res != VK_SUCCESS {
                return Err(map_vk_error(res))
            }
            let res = unsafe { (device.functions.image_functions.vk_bind_image_memory)(device.vk_device, vk_image, vk_device_memory, 0) };
            if res != VK_SUCCESS {
                return Err(map_vk_error(res))
            }
            
            Ok(VulkanImage {
                vk_image,
                vk_device_memory,
                allocated_size: vk_memory_requirements.size as usize
            })
    }
}

pub struct VulkanPhysicalDevice {
    vk_physical_device: VkPhysicalDevice,
    functions: PhysicalDeviceFunctions,
    vk_physical_device_properties: VkPhysicalDeviceProperties,
    queue_family_properties: Vec<VkQueueFamilyProperties>,
    vk_physical_device_memory_properties: VkPhysicalDeviceMemoryProperties
}

pub struct VulkanDevice<'a> {
    vk_device: VkDevice,
    physical_device: &'a VulkanPhysicalDevice,
    functions: VulkanFunctions,
    queues: Vec<VulkanQueue>
}

pub struct VulkanQueue {
    vk_queue: VkQueue,
    capabilities: QueueCapabilities
}

pub enum VulkanExtensions {
    
}

pub struct VulkanBuffer {
    vk_buffer: VkBuffer,
    vk_device_memory: VkDeviceMemory,
    allocated_size: usize,
    location: MemoryLocation
}

pub struct VulkanImage {
    vk_image: VkImage,
    vk_device_memory: VkDeviceMemory,
    allocated_size: usize
}

fn map_vk_error(res: VkResult) -> GraphicsError {
    match res {
        VK_NOT_READY => GraphicsError::BackendError { code: VK_NOT_READY.0, message: "VulkanBackend: VK_NOT_READY".to_string() },
        VK_TIMEOUT => GraphicsError::BackendError { code: VK_TIMEOUT.0, message: "VulkanBackend: VK_TIMEOUT".to_string() },
        VK_EVENT_SET => GraphicsError::BackendError { code: VK_EVENT_SET.0, message: "VulkanBackend: VK_EVENT_SET".to_string() },
        VK_EVENT_RESET => GraphicsError::BackendError { code: VK_EVENT_RESET.0, message: "VulkanBackend: VK_EVENT_RESET".to_string() },
        VK_INCOMPLETE => GraphicsError::BackendError { code: VK_INCOMPLETE.0, message: "VulkanBackend: VK_INCOMPLETE".to_string() },
        VK_ERROR_OUT_OF_HOST_MEMORY => GraphicsError::OutOfMemory("VulkanBackend: VK_ERROR_OUT_OF_HOST_MEMORY".to_string()),
        VK_ERROR_OUT_OF_DEVICE_MEMORY => GraphicsError::OutOfMemory("VulkanBackend: VK_ERROR_OUT_OF_DEVICE_MEMORY".to_string()),
        VK_ERROR_INITIALIZATION_FAILED => GraphicsError::InitializationFailed("VulkanBackend: VK_ERROR_INITIALIZATION_FAILED".to_string()),
        VK_ERROR_DEVICE_LOST => GraphicsError::BackendError { code: VK_ERROR_DEVICE_LOST.0, message: "VulkanBackend: VK_ERROR_DEVICE_LOST".to_string() },
        VK_ERROR_MEMORY_MAP_FAILED => GraphicsError::BackendError { code: VK_ERROR_MEMORY_MAP_FAILED.0, message: "VulkanBackend: VK_ERROR_MEMORY_MAP_FAILED".to_string() },
        VK_ERROR_LAYER_NOT_PRESENT => GraphicsError::BackendError { code: VK_ERROR_LAYER_NOT_PRESENT.0, message: "VulkanBackend: VK_ERROR_LAYER_NOT_PRESENT".to_string() },
        VK_ERROR_EXTENSION_NOT_PRESENT => GraphicsError::BackendError { code: VK_ERROR_EXTENSION_NOT_PRESENT.0, message: "VulkanBackend: VK_ERROR_EXTENSION_NOT_PRESENT".to_string() },
        VK_ERROR_FEATURE_NOT_PRESENT => GraphicsError::FeatureNotSupported("VulkanBackend: VK_ERROR_FEATURE_NOT_PRESENT".to_string()),
        VK_ERROR_INCOMPATIBLE_DRIVER => GraphicsError::BackendError { code: VK_ERROR_INCOMPATIBLE_DRIVER.0, message: "VulkanBackend: VK_ERROR_INCOMPATIBLE_DRIVER".to_string() },
        VK_ERROR_TOO_MANY_OBJECTS => GraphicsError::BackendError { code: VK_ERROR_TOO_MANY_OBJECTS.0, message: "VulkanBackend: VK_ERROR_TOO_MANY_OBJECTS".to_string() },
        VK_ERROR_FORMAT_NOT_SUPPORTED => GraphicsError::BackendError { code: VK_ERROR_FORMAT_NOT_SUPPORTED.0, message: "VulkanBackend: VK_ERROR_FORMAT_NOT_SUPPORTED".to_string() },
        VK_ERROR_FRAGMENTED_POOL => GraphicsError::BackendError { code: VK_ERROR_FRAGMENTED_POOL.0, message: "VulkanBackend: VK_ERROR_FRAGMENTED_POOL".to_string() },
        VK_ERROR_UNKNOWN => GraphicsError::BackendError { code: VK_ERROR_UNKNOWN.0, message: "VulkanBackend: Unknown error".to_string() },
        _ => GraphicsError::BackendError { code: -14, message: "VulkanBackend: Unknown error (error variant unkown)".to_string() }
    }
}