#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;

#[repr(C)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize
}

#[repr(C)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat
}