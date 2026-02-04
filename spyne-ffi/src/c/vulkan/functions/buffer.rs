use crate::c::vulkan::{constants::enums::result::VkResult, types::{base::VkDeviceSize, buffer::{VkBuffer, VkBufferCreateInfo}, device::VkDevice, instance::VkAllocationCallbacks, memory::{VkDeviceMemory, VkMemoryRequirements}}};

pub type VkCreateBuffer = unsafe extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkBufferCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_buffer: *mut VkBuffer,
) -> VkResult;

pub type VkDestroyBuffer = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    p_allocator: *const VkAllocationCallbacks,
);

pub type VkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    p_memory_requirements: *mut VkMemoryRequirements,
);

pub type VkBindBufferMemory = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

