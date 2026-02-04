use std::ffi::c_int;

use crate::c::macos::kqueue::types::{kevent, timespec};

unsafe extern "C" {
    pub fn kqueue() -> c_int;
    pub fn kevent(kq: c_int, changelist: *const kevent, nchanges: c_int, eventlist: *mut kevent, nevents: c_int, timeout: *const timespec) -> c_int;
}