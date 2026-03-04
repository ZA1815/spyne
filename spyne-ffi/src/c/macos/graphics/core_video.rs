use std::{ffi::{CString, c_void}, mem::transmute};

use crate::c::macos::general::{constants::RTLD_NOW, functions::{dlopen, dlsym}};

pub type CVDisplayLink = *mut c_void;
pub type CVReturn = i32;
pub type CVOptionFlags = u64;

pub type CVDisplayLinkCreateWithActiveCGDisplays = unsafe extern "C" fn(
    display_link_out: *mut CVDisplayLink
) -> CVReturn;

pub type CVDisplayLinkOutputCallback = unsafe extern "C" fn(
    display_link: CVDisplayLink,
    in_now: *const CVTimeStamp,
    in_output_time: *const CVTimeStamp,
    flags_in: CVOptionFlags,
    flags_out: *mut CVOptionFlags,
    display_link_context: *mut c_void
) -> CVReturn;

pub type CVDisplayLinkSetOutputCallback = unsafe extern "C" fn(
    display_link: CVDisplayLink,
    callback: CVDisplayLinkOutputCallback,
    user_info: *mut c_void
) -> CVReturn;

pub type CVDisplayLinkStart = unsafe extern "C" fn(
    display_link: CVDisplayLink
) -> CVReturn;

pub type CVDisplayLinkStop = unsafe extern "C" fn(
    display_link: CVDisplayLink
) -> CVReturn;

pub type CVDisplayLinkRelease = unsafe extern "C" fn(
    display_link: CVDisplayLink
);

pub type SMPTETime = [u8; 32];

#[repr(C)]
pub struct CVTimeStamp {
    pub version: u32,
    pub video_time_scale: i32,
    pub video_time: i64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub video_refresh_period: i64,
    pub smpte_time: SMPTETime,
    pub flags: u64,
    _reserved: u64
}

pub struct CoreVideoFunctions {
    pub cv_display_link_create_with_active_cg_displays: CVDisplayLinkCreateWithActiveCGDisplays,
    pub cv_display_link_set_output_callback: CVDisplayLinkSetOutputCallback,
    pub cv_display_link_start: CVDisplayLinkStart,
    pub cv_display_link_stop: CVDisplayLinkStop,
    pub cv_display_link_release: CVDisplayLinkRelease
}

impl CoreVideoFunctions {
    pub unsafe fn load() -> Self {
        let lib = unsafe { dlopen(CString::new("/System/Library/Frameworks/CoreVideo.framework/CoreVideo").unwrap().as_ptr(), RTLD_NOW) };
        let cv_display_link_create_with_active_cg_displays: CVDisplayLinkCreateWithActiveCGDisplays = unsafe { transmute(dlsym(lib, CString::new("CVDisplayLinkCreateWithActiveCGDisplays").unwrap().as_ptr())) };
        let cv_display_link_set_output_callback: CVDisplayLinkSetOutputCallback = unsafe { transmute(dlsym(lib, CString::new("CVDisplayLinkSetOutputCallback").unwrap().as_ptr())) };
        let cv_display_link_start: CVDisplayLinkStart = unsafe { transmute(dlsym(lib, CString::new("CVDisplayLinkStart").unwrap().as_ptr())) };
        let cv_display_link_stop: CVDisplayLinkStop = unsafe { transmute(dlsym(lib, CString::new("CVDisplayLinkStop").unwrap().as_ptr())) };
        let cv_display_link_release: CVDisplayLinkRelease = unsafe { transmute(dlsym(lib, CString::new("CVDisplayLinkRelease").unwrap().as_ptr())) };
        
        Self {
            cv_display_link_create_with_active_cg_displays,
            cv_display_link_set_output_callback,
            cv_display_link_start,
            cv_display_link_stop,
            cv_display_link_release
        }
    }
}

#[cfg(test)]
mod test {
    use std::hint::black_box;

    use crate::c::macos::graphics::core_video::CoreVideoFunctions;

    #[test]
    fn test_cv_funcs() {
        let cv_funcs = unsafe { CoreVideoFunctions::load() };
        
        black_box(&cv_funcs);
    }
}