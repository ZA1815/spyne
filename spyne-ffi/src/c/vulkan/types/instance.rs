use std::ffi::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkInstance(pub *mut c_void);

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PFN_vkAllocationFunction,
    pub pfn_reallocation: PFN_vkReallocationFunction,
    pub pfn_free: PFN_vkFreeFunction,
    pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
    pub pfn_internal_free: PFN_vkInternalFreeNotification,
}