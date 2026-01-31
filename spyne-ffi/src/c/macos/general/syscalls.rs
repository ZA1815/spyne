use core::ffi::{c_char, c_int, c_ulong};

#[link(name = "c")]
unsafe extern "C" {
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    pub fn fork() -> c_int;
    pub fn execve(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    pub fn _exit(status: c_int) -> !;
    pub fn setsid() -> c_int;
    pub fn __error() -> *mut c_int;
}