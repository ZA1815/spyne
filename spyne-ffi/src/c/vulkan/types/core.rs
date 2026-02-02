#[repr(C)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32
}

#[repr(C)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32
}

#[repr(C)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32
}