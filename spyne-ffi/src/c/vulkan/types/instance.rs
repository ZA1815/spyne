use std::ffi::c_void;

use std::ffi::c_char;

use crate::c::vulkan::{constants::{enums::{object_type::VkObjectType, structure_type::VkStructureType}, flags::instance_create::VkInstanceCreateFlagBits}, functions::func_pointers::{PfnVkAllocationFunction, PfnVkFreeFunction, PfnVkInternalAllocationNotification, PfnVkInternalFreeNotification, PfnVkReallocationFunction}, types::{base::VkFlags, command_buffer::VkDebugUtilsLabelEXT}};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkInstance(pub *mut c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkInstanceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub flags: VkInstanceCreateFlagBits,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: * const*const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: * const*const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkApplicationInfo {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PfnVkAllocationFunction,
    pub pfn_reallocation: PfnVkReallocationFunction,
    pub pfn_free: PfnVkFreeFunction,
    pub pfn_internal_allocation: PfnVkInternalAllocationNotification,
    pub pfn_internal_free: PfnVkInternalFreeNotification,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    // Hardcoded VkFlags here, make sure that the real flags type doesn't exist
    pub flags: VkFlags,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const VkDebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const VkDebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const VkDebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub s_type: VkStructureType,
    pub p_next: *const c_void,
    pub object_type: VkObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
}

