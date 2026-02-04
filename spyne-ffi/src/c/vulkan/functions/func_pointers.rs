use std::ffi::{c_char, c_void};

use crate::c::vulkan::{constants::{enums::{debug_report_object_type_ext::VkDebugReportObjectTypeEXT, internal_allocation_type::VkInternalAllocationType, system_allocation_scope::VkSystemAllocationScope}, flags::{debug_report_ext::VkDebugReportFlagBitsEXT, debug_utils_message_severity_ext::VkDebugUtilsMessageSeverityFlagBitsEXT, debug_utils_message_type_ext::VkDebugUtilsMessageTypeFlagBitsEXT}}, types::{base::VkBool32, device::VkFaultData, instance::{VkDebugUtilsMessengerCallbackDataEXT, VkInstance}, memory::VkDeviceMemoryReportCallbackDataEXT}};

pub type PfnVkInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

pub type PfnVkInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

pub type PfnVkReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

pub type PfnVkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

pub type PfnVkFreeFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_memory: *mut c_void,
);

pub type PfnVkVoidFunction = unsafe extern "system" fn(
);

pub type PfnVkDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: VkDebugReportFlagBitsEXT,
    object_type: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
    p_user_data: *mut c_void,
) -> VkBool32;

pub type PfnVkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagBitsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut c_void,
) -> VkBool32;

pub type PfnVkFaultCallbackFunction = unsafe extern "system" fn(
    unrecorded_faults: VkBool32,
    fault_count: u32,
    p_faults: *const VkFaultData,
);

pub type PfnVkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const VkDeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut c_void,
);

pub type PfnVkGetInstanceProcAddrLUNARG = unsafe extern "system" fn(
    instance: VkInstance,
    p_name: *const c_char,
) -> PfnVkVoidFunction;

