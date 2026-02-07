mod buffer;
mod command_buffer;
mod device;
pub mod func_pointers;
mod image;
mod instance;
mod memory;
mod physical_device;
mod pipeline;
mod queue;
mod render_pass;
mod shader;
mod surface;
mod swapchain;
mod sync;

use std::{ffi::CString, mem::transmute, ptr::null_mut};

use spyne_macros::VulkanFunctions;

use crate::c::{linux::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}}, vulkan::{functions::{buffer::{VkBindBufferMemory, VkCreateBuffer, VkDestroyBuffer, VkGetBufferMemoryRequirements}, command_buffer::{VkAllocateCommandBuffers, VkBeginCommandBuffer, VkCmdBeginRenderPass, VkCmdBindPipeline, VkCmdBindVertexBuffers, VkCmdDraw, VkCmdEndRenderPass, VkCmdSetScissor, VkCmdSetViewport, VkCreateCommandPool, VkDestroyCommandPool, VkEndCommandBuffer, VkFreeCommandBuffers, VkResetCommandBuffer}, device::{ VkDestroyDevice, VkDeviceWaitIdle, VkGetDeviceQueue}, image::{VkBindImageMemory, VkCreateImage, VkCreateImageView, VkDestroyImage, VkDestroyImageView, VkGetImageMemoryRequirements}, instance::{VkCreateDevice, VkCreateInstance, VkEnumeratePhysicalDevices, VkGetDeviceProcAddr, VkGetInstanceProcAddr}, memory::{VkAllocateMemory, VkFreeMemory, VkMapMemory, VkUnmapMemory}, physical_device::{VkGetPhysicalDeviceFeatures, VkGetPhysicalDeviceMemoryProperties, VkGetPhysicalDeviceProperties, VkGetPhysicalDeviceQueueFamilyProperties}, pipeline::{VkCreateGraphicsPipelines, VkCreatePipelineLayout, VkDestroyPipeline, VkDestroyPipelineLayout}, queue::{VkQueuePresentKHR, VkQueueSubmit, VkQueueWaitIdle}, render_pass::{VkCreateFramebuffer, VkCreateRenderPass, VkDestroyFramebuffer, VkDestroyRenderPass}, shader::{VkCreateShaderModule, VkDestroyShaderModule}, surface::{VkDestroySurfaceKHR, VkGetPhysicalDeviceSurfaceCapabilitiesKHR, VkGetPhysicalDeviceSurfaceFormatsKHR, VkGetPhysicalDeviceSurfacePresentModesKHR, VkGetPhysicalDeviceSurfaceSupportKHR}, swapchain::{VkAcquireNextImageKHR, VkCreateSwapchainKHR, VkDestroySwapchainKHR, VkGetSwapchainImagesKHR}, sync::{VkCreateFence, VkCreateSemaphore, VkDestroyFence, VkDestroySemaphore, VkResetFences, VkWaitForFences}}, types::{device::VkDevice, instance::VkInstance}}};

#[cfg(target_os = "linux")]
use crate::c::vulkan::functions::surface::VkCreateWaylandSurfaceKHR;

pub struct VulkanFunctions {
    pub entry_functions: EntryFunctions,
    pub buffer_functions: BufferFunctions,
    pub command_buffer_functions: CommandBufferFunctions,
    pub device_functions: DeviceFunctions,
    pub image_functions: ImageFunctions,
    pub instance_functions: InstanceFunctions,
    pub memory_functions: MemoryFunctions,
    pub physical_device_functions: PhysicalDeviceFunctions,
    pub pipeline_functions: PipelineFunctions,
    pub queue_functions: QueueFunctions,
    pub render_pass_functions: RenderPassFunctions,
    pub shader_functions: ShaderFunctions,
    pub surface_functions: SurfaceFunctions,
    pub swapchain_functions: SwapchainFunctions,
    pub sync_functions: SyncFunctions
}

impl VulkanFunctions {
    pub unsafe fn load(entry_functions: EntryFunctions, instance: VkInstance, device: VkDevice) -> Self {
        let vk_get_instance_proc_addr = entry_functions.vk_get_instance_proc_addr;
        let instance_functions = unsafe { InstanceFunctions::load(vk_get_instance_proc_addr, instance) };
        let physical_device_functions = unsafe { PhysicalDeviceFunctions::load(vk_get_instance_proc_addr, instance) };
        let surface_functions = unsafe { SurfaceFunctions::load(vk_get_instance_proc_addr, instance) };
        let vk_get_device_proc_addr = instance_functions.vk_get_device_proc_addr;
        let buffer_functions = unsafe { BufferFunctions::load(vk_get_device_proc_addr, device) };
        let command_buffer_functions = unsafe { CommandBufferFunctions::load(vk_get_device_proc_addr, device) };
        let device_functions = unsafe { DeviceFunctions::load(vk_get_device_proc_addr, device) };
        let image_functions = unsafe { ImageFunctions::load(vk_get_device_proc_addr, device) };
        let memory_functions = unsafe { MemoryFunctions::load(vk_get_device_proc_addr, device) };
        let pipeline_functions = unsafe { PipelineFunctions::load(vk_get_device_proc_addr, device) };
        let queue_functions = unsafe { QueueFunctions::load(vk_get_device_proc_addr, device) };
        let render_pass_functions = unsafe { RenderPassFunctions::load(vk_get_device_proc_addr, device) };
        let shader_functions = unsafe { ShaderFunctions::load(vk_get_device_proc_addr, device) };
        let swapchain_functions = unsafe { SwapchainFunctions::load(vk_get_device_proc_addr, device) };
        let sync_functions = unsafe { SyncFunctions::load(vk_get_device_proc_addr, device) };
        
        Self {
            entry_functions,
            buffer_functions,
            command_buffer_functions,
            device_functions,
            image_functions,
            instance_functions,
            memory_functions,
            physical_device_functions,
            pipeline_functions,
            queue_functions,
            render_pass_functions,
            shader_functions,
            surface_functions,
            swapchain_functions,
            sync_functions
        }
    }
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
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct BufferFunctions {
    #[vulkan(name = "vkCreateBuffer")]
    pub vk_create_buffer: VkCreateBuffer,
    
    #[vulkan(name = "vkDestroyBuffer")]
    pub vk_destroy_buffer: VkDestroyBuffer,
    
    #[vulkan(name = "vkGetBufferMemoryRequirements")]
    pub vk_get_buffer_memory_requirements: VkGetBufferMemoryRequirements,
    
    #[vulkan(name = "vkBindBufferMemory")]
    pub vk_bind_buffer_memory: VkBindBufferMemory
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct CommandBufferFunctions {
    #[vulkan(name = "vkCreateCommandPool")]
    pub vk_create_command_pool: VkCreateCommandPool,
    
    #[vulkan(name = "vkDestroyCommandPool")]
    pub vk_destroy_command_pool: VkDestroyCommandPool,
    
    #[vulkan(name = "vkAllocateCommandBuffers")]
    pub vk_allocate_command_buffers: VkAllocateCommandBuffers,
    
    #[vulkan(name = "vkFreeCommandBuffers")]
    pub vk_free_command_buffers: VkFreeCommandBuffers,
    
    #[vulkan(name = "vkBeginCommandBuffer")]
    pub vk_begin_command_buffer: VkBeginCommandBuffer,
    
    #[vulkan(name = "vkEndCommandBuffer")]
    pub vk_end_command_buffer: VkEndCommandBuffer,
    
    #[vulkan(name = "vkResetCommandBuffer")]
    pub vk_reset_command_buffer: VkResetCommandBuffer,
    
    #[vulkan(name = "vkCmdBeginRenderPass")]
    pub vk_cmd_begin_render_pass: VkCmdBeginRenderPass,
    
    #[vulkan(name = "vkCmdEndRenderPass")]
    pub vk_cmd_end_render_pass: VkCmdEndRenderPass,
    
    #[vulkan(name = "vkCmdBindPipeline")]
    pub vk_cmd_bind_pipeline: VkCmdBindPipeline,
    
    #[vulkan(name = "vkCmdBindVertexBuffers")]
    pub vk_cmd_bind_vertex_buffers: VkCmdBindVertexBuffers,
    
    #[vulkan(name = "vkCmdSetViewport")]
    pub vk_cmd_set_viewport: VkCmdSetViewport,
    
    #[vulkan(name = "vkCmdSetScissor")]
    pub vk_cmd_set_scissor: VkCmdSetScissor,
    
    #[vulkan(name = "vkCmdDraw")]
    pub vk_cmd_draw: VkCmdDraw
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct DeviceFunctions {
    #[vulkan(name = "vkGetDeviceQueue")]
    pub vk_get_device_queue: VkGetDeviceQueue,
    
    #[vulkan(name = "vkDeviceWaitIdle")]
    pub vk_device_wait_idle: VkDeviceWaitIdle,
    
    #[vulkan(name = "vkDestroyDevice")]
    pub vk_destroy_device: VkDestroyDevice
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct ImageFunctions {
    #[vulkan(name = "vkCreateImage")]
    pub vk_create_image: VkCreateImage,
    
    #[vulkan(name = "vkDestroyImage")]
    pub vk_destroy_image: VkDestroyImage,
    
    #[vulkan(name = "vkGetImageMemoryRequirements")]
    pub vk_get_image_memory_requirements: VkGetImageMemoryRequirements,
    
    #[vulkan(name = "vkBindImageMemory")]
    pub vk_bind_image_memory: VkBindImageMemory,
    
    #[vulkan(name = "vkCreateImageView")]
    pub vk_create_image_view: VkCreateImageView,
   
    #[vulkan(name = "vkDestroyImageView")] 
    pub vk_destroy_image_view: VkDestroyImageView
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkInstance, loader = VkGetInstanceProcAddr)]
pub struct InstanceFunctions {
    #[vulkan(name = "vkEnumeratePhysicalDevices")]
    pub vk_enumerate_physical_devices: VkEnumeratePhysicalDevices,
    
    #[vulkan(name = "vkCreateDevice")]
    pub vk_create_device: VkCreateDevice,
    
    #[vulkan(name = "vkGetDeviceProcAddr")]
    pub vk_get_device_proc_addr: VkGetDeviceProcAddr,
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct MemoryFunctions {
    #[vulkan(name = "vkAllocateMemory")]
    pub vk_allocate_memory: VkAllocateMemory,
    
    #[vulkan(name = "vkFreeMemory")]
    pub vk_free_memory: VkFreeMemory,
    
    #[vulkan(name = "vkMapMemory")]
    pub vk_map_memory: VkMapMemory,
    
    #[vulkan(name = "vkUnmapMemory")]
    pub vk_unmap_memory: VkUnmapMemory
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkInstance, loader = VkGetInstanceProcAddr)]
pub struct PhysicalDeviceFunctions {
    #[vulkan(name = "vkGetPhysicalDeviceProperties")]
    pub vk_get_physical_device_properties: VkGetPhysicalDeviceProperties,
    
    #[vulkan(name = "vkGetPhysicalDeviceFeatures")]
    pub vk_get_physical_device_features: VkGetPhysicalDeviceFeatures,
    
    #[vulkan(name = "vkGetPhysicalDeviceMemoryProperties")]
    pub vk_get_physical_device_memory_properties: VkGetPhysicalDeviceMemoryProperties,
    
    #[vulkan(name = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub vk_get_physical_device_queue_family_properties: VkGetPhysicalDeviceQueueFamilyProperties
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct PipelineFunctions {
    #[vulkan(name = "vkCreatePipelineLayout")]
    pub vk_create_pipeline_layout: VkCreatePipelineLayout,
    
    #[vulkan(name = "vkDestroyPipelineLayout")]
    pub vk_destroy_pipeline_layout: VkDestroyPipelineLayout,
    
    #[vulkan(name = "vkCreateGraphicsPipelines")]
    pub vk_create_graphics_pipelines: VkCreateGraphicsPipelines,
    
    #[vulkan(name = "vkDestroyPipeline")]
    pub vk_destroy_pipeline: VkDestroyPipeline
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct QueueFunctions {
    #[vulkan(name = "vkQueueSubmit")]
    pub vk_queue_submit: VkQueueSubmit,
    
    #[vulkan(name = "vkQueueWaitIdle")]
    pub vk_queue_wait_idle: VkQueueWaitIdle,
    
    #[vulkan(name = "vkQueuePresentKHR")]
    pub vk_queue_present_khr: VkQueuePresentKHR
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct RenderPassFunctions {
    #[vulkan(name = "vkCreateRenderPass")]
    pub vk_create_render_pass: VkCreateRenderPass,
    
    #[vulkan(name = "vkDestroyRenderPass")]
    pub vk_destroy_render_pass: VkDestroyRenderPass,
    
    #[vulkan(name = "vkCreateFramebuffer")]
    pub vk_create_framebuffer: VkCreateFramebuffer,
    
    #[vulkan(name = "vkDestroyFramebuffer")]
    pub vk_destroy_framebuffer: VkDestroyFramebuffer
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct ShaderFunctions {
    #[vulkan(name = "vkCreateShaderModule")]
    pub vk_create_shader_module: VkCreateShaderModule,
    
    #[vulkan(name = "vkDestroyShaderModule")]
    pub vk_destroy_shader_module: VkDestroyShaderModule
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkInstance, loader = VkGetInstanceProcAddr)]
pub struct SurfaceFunctions {
    #[vulkan(name = "vkDestroySurfaceKHR")]
    pub vk_destroy_surface_khr: VkDestroySurfaceKHR,
    
    #[vulkan(name = "vkGetPhysicalDeviceSurfaceSupportKHR")]
    pub vk_get_physical_device_surface_support_khr: VkGetPhysicalDeviceSurfaceSupportKHR,
    
    #[vulkan(name = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
    pub vk_get_physical_device_surface_capabilities_khr: VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    
    #[vulkan(name = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
    pub vk_get_physical_device_surface_formats_khr: VkGetPhysicalDeviceSurfaceFormatsKHR,
    
    #[vulkan(name = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
    pub vk_get_physical_device_surface_present_modes_khr: VkGetPhysicalDeviceSurfacePresentModesKHR,
    
    #[cfg(target_os = "linux")]
    #[vulkan(name = "vkWaylandSurfaceInfoKHR")]
    pub vk_create_wayland_surface_khr: VkCreateWaylandSurfaceKHR
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct SwapchainFunctions {
    #[vulkan(name = "vkCreateSwapchainKHR")]
    pub vk_create_swapchain_khr: VkCreateSwapchainKHR,
    
    #[vulkan(name = "vkDestroySwapchainKHR")]
    pub vk_destroy_swapchain_khr: VkDestroySwapchainKHR,
    
    #[vulkan(name = "vkGetSwapchainImagesKHR")]
    pub vk_get_swapchain_images_khr: VkGetSwapchainImagesKHR,
    
    #[vulkan(name = "vkAcquireNextImageKHR")]
    pub vk_acquire_next_image_khr: VkAcquireNextImageKHR
}

#[derive(VulkanFunctions)]
#[vulkan(handle = VkDevice, loader = VkGetDeviceProcAddr)]
pub struct SyncFunctions {
    #[vulkan(name = "vkCreateFence")]
    pub vk_create_fence: VkCreateFence,
    
    #[vulkan(name = "vkDestroyFence")]
    pub vk_destroy_fence: VkDestroyFence,
    
    #[vulkan(name = "vkWaitForFences")]
    pub vk_wait_for_fences: VkWaitForFences,
    
    #[vulkan(name = "vkResetFences")]
    pub vk_reset_fences: VkResetFences,
    
    #[vulkan(name = "vkCreateSemaphore")]
    pub vk_create_semaphore: VkCreateSemaphore,
    
    #[vulkan(name = "vkDestroySemaphore")]
    pub vk_destroy_semaphore: VkDestroySemaphore
}

#[cfg(test)]
mod test {
    use std::ptr::{null, null_mut};

    use crate::c::vulkan::{constants::{enums::{result::VK_SUCCESS, structure_type::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO}, flags::instance_create::VkInstanceCreateFlagBits}, functions::{EntryFunctions, InstanceFunctions}, types::{instance::{VkInstance, VkInstanceCreateInfo}, physical_device::VkPhysicalDevice}};

    #[test]
    fn test_vulkan_funcs() {
        let entry_funcs = unsafe { EntryFunctions::load() };
        let mut instance = VkInstance(null_mut());
        let instance_info = VkInstanceCreateInfo {
            s_type: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: null(),
            flags: VkInstanceCreateFlagBits(0),
            p_application_info: null(),
            enabled_layer_count: 0,
            pp_enabled_layer_names: null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: null()
        };
        let res_init = unsafe { (entry_funcs.vk_create_instance)(&instance_info, null(), &mut instance) };
        if res_init.0 < 0 {
            panic!("Vulkan Failed.");
        }
        let instance_funcs = unsafe { InstanceFunctions::load(entry_funcs.vk_get_instance_proc_addr, instance) };
        let mut device_count: u32 = 1;
        let mut physical_device = VkPhysicalDevice(null_mut());
        let res_device = unsafe { (instance_funcs.vk_enumerate_physical_devices)(instance, &mut device_count, &mut physical_device) };
        if res_device.0 < 0 {
            panic!("Vulkan Failed.");
        }
        assert_eq!(res_init, VK_SUCCESS);
        assert_eq!(res_device, VK_SUCCESS);
    }
}