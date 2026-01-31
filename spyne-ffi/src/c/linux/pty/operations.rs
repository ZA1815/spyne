extern crate alloc;
use alloc::vec::Vec;

use crate::c::linux::{general::{constants::O_RDWR, syscalls::{_exit, close, dup2, execve, fork, ioctl, open, setsid}}, pty::constants::{O_NOCTTY, TIOCGPTN, TIOCSCTTY, TIOCSPTLCK}};

pub fn create_pty() -> (i32, i32) {
    let master_flags = O_RDWR | O_NOCTTY;
    let master_fd = unsafe { open(b"/dev/ptmx\0" as *const u8, master_flags, 0) };
    if master_fd < 0 {
        panic!("create_pty: open using /dev/ptmx failed.")
    }
    
    let mut pty_num: i32 = 0;
    let res = unsafe { ioctl(master_fd as i32, TIOCGPTN, &mut pty_num as *mut _ as u64) };
    if res < 0 {
        panic!("create_pty: ioctl using TIOCGPTN failed.");
    }
    
    let unlock: i32 = 0;
    let res = unsafe { ioctl(master_fd as i32, TIOCSPTLCK, &unlock as *const _ as u64) };
    if res < 0 {
        panic!("create_pty: ioctl using TIOCSPTLCK failed.");
    }
    
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"/dev/pts/");
    let start = buf.len();
    let mut n = pty_num;
    if n == 0 {
        buf.push(b'0');
    }
    else {
        while n > 0 {
            buf.push(b'0' + (n % 10) as u8);
            n /= 10;
        }
        buf[start..].reverse();
    }
    buf.push(0);
    
    let slave_fd = unsafe { open(buf.as_ptr(), O_RDWR, 0) };
    if slave_fd < 0 {
        panic!("create_pty: open using /dev/pts/{} failed.", pty_num);
    }
    
    (master_fd as i32, slave_fd as i32)
}

pub fn spawn(master_fd: i32, slave_fd: i32, path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i32 {
    let pid = unsafe { fork() };
    if pid == 0 {
        let res = unsafe { setsid() };
        if res < 0 {
            unsafe { _exit(1) }
        }
        
        let res = unsafe { dup2(slave_fd, 0) };
        if res < 0 {
            unsafe { _exit(1) }
        }
        
        let res = unsafe { dup2(slave_fd, 1) };
        if res < 0 {
            unsafe { _exit(1) }
        }
        
        let res = unsafe { dup2(slave_fd, 2) };
        if res < 0 {
            unsafe { _exit(1) }
        }
            
        let res = unsafe { ioctl(slave_fd, TIOCSCTTY, 0) };
        if res < 0 {
            unsafe { _exit(1) }
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
            unsafe { _exit(1) }
        }
        
        return pid as i32;
    }
    else if pid > 0 {
        let res = unsafe { close(slave_fd) };
        if res < 0 {
            panic!("spawn: Parent failed to close slave_fd");
        }
        
        return pid as i32;
    }
    else {
        panic!("spawn: fork failed.");
    }
}