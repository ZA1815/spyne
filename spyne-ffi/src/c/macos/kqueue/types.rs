use core::ffi::{c_long, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct kevent {
    pub ident: usize,
    pub filter: i16,
    pub flags: u16,
    pub fflags: u32,
    pub data: i64,
    pub udata: *mut c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long
}