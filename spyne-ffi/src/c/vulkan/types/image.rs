use std::ffi::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkImage(pub *mut c_void);


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkImageView(pub *mut c_void);

#[repr(C)]
pub struct VkImageCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkImageCreateFlags,
    pub image_type: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: VkImageLayout,
}

#[repr(C)]
pub struct VkImageViewCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub view_type: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[repr(C)]
pub struct VkImageSubresourceRange {
    pub aspect_mask: VkImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

