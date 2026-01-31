#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64
}