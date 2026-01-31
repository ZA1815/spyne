// Flags
pub const EV_ADD: u16 = 0x0001;
pub const EV_DELETE: u16 = 0x0002;
pub const EV_ENABLE: u16 = 0x0004;
pub const EV_DISABLE: u16 = 0x0008;
pub const EV_CLEAR: u16 = 0x0020;
pub const EV_ERROR: u16 = 0x4000;
pub const EV_EOF: u16 = 0x8000;

// Filters
pub const EVFILT_READ: i16 = -1;
pub const EVFILT_WRITE: i16 = -2;