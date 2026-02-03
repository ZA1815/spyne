#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkMemoryPropertyFlagBits(pub u32);

pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 0);
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 1);
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 2);
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 3);
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1 << 4);
