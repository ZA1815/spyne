use core::ffi::c_ulong;

// Flags
pub const O_RDONLY: i32 = 0x0;
pub const O_WRONLY: i32 = 0x1;
pub const O_RDWR: i32 = 0x2;
pub const O_NOCTTY: i32 = 0x20000;
pub const O_CLOEXEC: i32 = 0x1000000;

// IOCTL Request Codes
pub const TIOCSCTTY: c_ulong = 0x20007461; // Terminal IOCtl Set Controlling TTY
pub const TIOCSPTLCK: c_ulong = 0x80087467; // Terminal IOCtl Set PTy LoCK
pub const TIOCGPTN: c_ulong = 0x40087468; // Terminal IOCtl Get PTy Number