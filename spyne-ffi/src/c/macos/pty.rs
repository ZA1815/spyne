use crate::c::macos::constants::{O_NOCTTY, O_RDWR, TIOCSCTTY};
use crate::c::macos::syscalls::{_exit, close, dup2, execve, fork, grantpt, ioctl, open, posix_openpt, ptsname, setsid, unlockpt};

pub fn create_pty() -> (i32, i32) {
    let master_flags = O_RDWR | O_NOCTTY;
    let master_fd = unsafe { posix_openpt(master_flags) };
    if master_fd < 0 {
        panic!("create_pty: posix_openpt failed.");
    }
    
    let res = unsafe { grantpt(master_fd) };
    if res < 0 {
        panic!("create_pty: grantpt failed.");
    }
    
    let res = unsafe { unlockpt(master_fd) };
    if res < 0 {
        panic!("create_pty: unlockpt failed.");
    }
    
    let slave_path = unsafe { ptsname(master_fd) };
    if slave_path.is_null() {
        panic!("create_pty: ptsname failed.");
    }
    
    let slave_fd = unsafe { open(slave_path, O_RDWR, 0) };
    if slave_fd < 0 {
        panic!("create_pty: open failed.");
    }
    
    (master_fd, slave_fd as i32)
}

pub fn spawn(master_fd: i32, slave_fd: i32, path: *const i8, argv: *const *const i8, envp: *const *const i8) -> i32 {
    let pid = unsafe { fork() };
    if pid == 0 {
        let res = unsafe { setsid() };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        let res = unsafe { dup2(slave_fd, 0) };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        let res = unsafe { dup2(slave_fd, 1) };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        let res = unsafe { dup2(slave_fd, 2) };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        let res = unsafe { ioctl(slave_fd, TIOCSCTTY) };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        let res = unsafe { close(master_fd) };
        if res < 0 {
            unsafe { _exit(1) }
        }
        
        let res = unsafe { close(slave_fd) };
        if res < 0 {
            unsafe { _exit(1) }
        }
        
        let res = unsafe { execve(path, argv, envp) };
        if res < 0 {
            unsafe { _exit(1) };
        }
        
        return pid;
    }
    else if pid > 0 {
        let res = unsafe { close(slave_fd) };
        if res < 0 {
            panic!("spawn: Parent failed to close slave_fd");
        }
        
        return pid
    }
    else {
        panic!("spawn: fork failed");
    }
}