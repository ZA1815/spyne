#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFrontFace(u32);

pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = VkFrontFace(0);
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = VkFrontFace(1);
