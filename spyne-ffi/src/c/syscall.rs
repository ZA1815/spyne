use core::arch::asm;

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