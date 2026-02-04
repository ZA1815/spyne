use std::ffi::c_ulong;

// Flags
pub const O_NOCTTY: i32 = 0x20000;

// IOCTL Request Codes
pub const TIOCSCTTY: c_ulong = 0x20007461; // Terminal IOCtl Set Controlling TTY
pub const TIOCSPTLCK: c_ulong = 0x80087467; // Terminal IOCtl Set PTy LoCK
pub const TIOCGPTN: c_ulong = 0x40087468; // Terminal IOCtl Get PTy Number