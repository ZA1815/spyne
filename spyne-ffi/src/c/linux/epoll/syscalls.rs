use crate::c::linux::{epoll::{constants::{SYS_EPOLL_CREATE1, SYS_EPOLL_CTL, SYS_EPOLL_WAIT}, types::epoll_event}, general::syscalls::{syscall1, syscall4}};

pub unsafe fn epoll_create1(flags: i32) -> isize {
    unsafe { syscall1(SYS_EPOLL_CREATE1 as u64, flags as u64) }
}

pub unsafe fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut epoll_event) -> isize {
    unsafe { syscall4(SYS_EPOLL_CTL as u64, epfd as u64, op as u64, fd as u64, event as u64) }
}

pub unsafe fn epoll_wait(epfd: i32, events: *mut epoll_event, max_events: i32, timeout: i32) -> isize {
    unsafe { syscall4(SYS_EPOLL_WAIT as u64, epfd as u64, events as u64, max_events as u64, timeout as u64) }
}