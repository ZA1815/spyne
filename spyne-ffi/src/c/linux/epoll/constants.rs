// Syscall Numbers
pub const SYS_EPOLL_WAIT: i64 = 232;
pub const SYS_EPOLL_CTL: i64 = 233;
pub const SYS_EPOLL_CREATE1: i64 = 291;

// Control Operations
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;

// Event Flags
pub const EPOLLIN: u32 = 0x001;
pub const EPOLLPRI: u32 = 0x002;
pub const EPOLLOUT: u32 = 0x004;
pub const EPOLLERR: u32 = 0x008;
pub const EPOLLHUP: u32 = 0x010;