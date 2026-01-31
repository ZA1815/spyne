// Syscall numbers
pub const SYS_OPEN: u32 = 0x2;
pub const SYS_CLOSE: u32 = 0x3;
pub const SYS_IOCTL: u32 = 0x10;
pub const SYS_DUP2: u32 = 0x21;
pub const SYS_FORK: u32 = 0x39;
pub const SYS_EXECVE: u32 = 0x3B;
pub const SYS_EXIT :u32 = 0x3C;
pub const SYS_SETSID: u32 = 0x70;

// Syscall Flags
pub const O_RDONLY: i32 = 0x0;
pub const O_WRONLY: i32 = 0x1;
pub const O_RDWR: i32 = 0x2;
pub const O_CLOEXEC: i32 = 0x80000;

// Dlopen Flags
pub const RTLD_LOCAL: i32 = 0;
pub const RTLD_LAZY: i32 = 0x00001;
pub const RTLD_NOW: i32 = 0x00002;
pub const RTLD_NOLOAD: i32 = 0x00004;
pub const RTLD_DEEPBIND: i32 = 0x00008;
pub const RTLD_GLOBAL: i32 = 0x00100;
pub const RTLD_NODELETE: i32 = 0x01000;