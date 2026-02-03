use std::ffi::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkShaderModule(pub *mut c_void);

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub code_size: size_t,
    pub p_code: *const u32,
}

