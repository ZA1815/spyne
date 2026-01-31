// Flags
pub const O_NOCTTY: i32 = 0x100;

// IOCTL Request Codes
pub const TIOCSCTTY: u32 = 0x540E; // Terminal IOCtl Set Controlling TTY
pub const TIOCSPTLCK: u32 = 0x40045431; // Terminal IOCtl Set PTy LoCK
pub const TIOCGPTN: u32 = 0x80045430; // Terminal IOCtl Get PTy Number