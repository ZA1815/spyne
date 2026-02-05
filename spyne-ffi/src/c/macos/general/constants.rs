// Flags
pub const O_RDONLY: i32 = 0x0;
pub const O_WRONLY: i32 = 0x1;
pub const O_RDWR: i32 = 0x2;
pub const O_CLOEXEC: i32 = 0x1000000;

// Dlopen Flags
pub const RTLD_LAZY: i32 = 0x1;
pub const RTLD_NOW: i32 = 0x2;
pub const RTLD_LOCAL: i32 = 0x4;
pub const RTLD_GLOBAL: i32 = 0x8;
pub const RTLD_NOLOAD: i32 = 0x10;
pub const RTLD_NODELETE: i32 = 0x80;