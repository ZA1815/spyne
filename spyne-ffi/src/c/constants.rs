// Syscall numbers
pub const SYS_OPEN: u32 = 0x2;
pub const SYS_CLOSE: u32 = 0x3;
pub const SYS_IOCTL: u32 = 0x10;
pub const SYS_DUP2: u32 = 0x21;
pub const SYS_FORK: u32 = 0x39;
pub const SYS_EXECVE: u32 = 0x3B;

// Flags
pub const O_RDONLY: i32 = 0x0;
pub const O_WRONLY: i32 = 0x1;
pub const O_RDWR: i32 = 0x2;
pub const O_NOCTTY: i32 = 0x100;
pub const O_CLOEXEC: i32 = 0x80000;

// IOCTL Request Codes
pub const TIOCGPTN: u32 = 0x80045430;
pub const TIOCSPTLCK: u32 = 0x40045431;