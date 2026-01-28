use core::arch::asm;

use crate::c::constants::{SYS_CLOSE, SYS_DUP2, SYS_EXECVE, SYS_FORK, SYS_IOCTL, SYS_OPEN};

pub unsafe fn open(path: *const u8, flags: i32, mode: i32) ->  isize {
    unsafe { syscall3(SYS_OPEN as u64, path as u64, flags as u64, mode as u64) }
}

pub unsafe fn close(fd: i32) -> isize {
    unsafe { syscall1(SYS_CLOSE as u64, fd as u64) }
}

pub unsafe fn ioctl(fd: i32, request: u32, arg: u64) -> isize {
    unsafe { syscall3(SYS_IOCTL as u64, fd as u64, request as u64, arg) }
}

pub unsafe fn dup2(src_fd: i32, dest_fd: i32) -> isize {
    unsafe { syscall2(SYS_DUP2 as u64, src_fd as u64, dest_fd as u64) }
}

pub unsafe fn fork() -> isize {
    unsafe { syscall0(SYS_FORK as u64) }
}

pub unsafe fn execve(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> isize {
    unsafe { syscall3(SYS_EXECVE as u64, path as u64, argv as u64, envp as u64) }
}

pub unsafe fn syscall0(rax: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall1(rax: u64, rdi: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall2(rax: u64, rdi: u64, rsi: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            in("rsi") rsi,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall3(rax: u64, rdi: u64, rsi: u64, rdx: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall4(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("r10") r10,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall5(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("r10") r10,
            in("r8") r8,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}

pub unsafe fn syscall6(rax: u64, rdi: u64, rsi: u64, rdx: u64, r10: u64, r8: u64, r9: u64) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") rax,
            in("rdi") rdi,
            in("rsi") rsi,
            in("rdx") rdx,
            in("r10") r10,
            in("r8") r8,
            in("r9") r9,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack)
        );
    }
    
    ret
}