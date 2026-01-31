use std::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn posix_openpt(flags: i32) -> c_int;
    pub fn grantpt(fd: i32) -> c_int;
    pub fn unlockpt(fd: i32) -> c_int;
    pub fn ptsname(fd: i32) -> *const c_char;
}