use std::{ptr::null_mut, time::Duration};

use spyne_ffi::c::linux::epoll::{constants::{EPOLL_CLOEXEC, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD, EPOLLERR, EPOLLHUP, EPOLLIN, EPOLLOUT}, syscalls::{epoll_create1, epoll_ctl, epoll_wait}, types::epoll_event};

use crate::io::poll::{FilterType, FlagType, Interests, PollEvent, Poller};

pub struct Epoll {
    fd: i32
}

impl Epoll {
    pub fn new() -> Self {
        Self {
            fd: unsafe { epoll_create1(EPOLL_CLOEXEC) as i32 }
        }
    }
    
    pub fn with_flags(flags: i32) -> Self {
        Self {
            fd: unsafe { epoll_create1(flags) as i32 }
        }
    }
}

impl Poller for Epoll {
    type Source = i32;
    type Event = EpollEvent;
    
    fn add(&mut self, source: Self::Source, interests: Interests) {
        let mut event = create_epoll_event(source, interests);
        let res = unsafe { epoll_ctl(self.fd, EPOLL_CTL_ADD, source, &mut event) };
        if res < 0 {
            // Add error later
        }
    }
    
    fn modify(&mut self, source: Self::Source, interests: Interests) {
        let mut event = create_epoll_event(source, interests);
        let res = unsafe { epoll_ctl(self.fd, EPOLL_CTL_MOD, source, &mut event) };
        if res < 0 {
            // Add error later
        }
    }
    
    fn remove(&mut self, source: Self::Source) {
        let res = unsafe { epoll_ctl(self.fd, EPOLL_CTL_DEL, source, null_mut()) };
        if res < 0 {
            // Add error later
        }
    }
    
    fn wait(&mut self, timeout: Duration, buffer: &mut Vec<Self::Event>, max_events: i32) -> Result<usize, super::PollError> {
        let mut epoll_buffer = Vec::<epoll_event>::with_capacity(max_events as usize);
        let num_events = unsafe { epoll_wait(self.fd, epoll_buffer.as_mut_ptr(), max_events, timeout.as_millis() as i32) };
        if num_events < 0 {
            // Add error later
        }
        unsafe { epoll_buffer.set_len(num_events as usize) };
        buffer.clear();
        
        for item in epoll_buffer {
            let is_read = item.events & EPOLLIN != 0;
            let is_write = item.events & EPOLLOUT != 0;
            let filter_type = if is_read && is_write {
                FilterType::ReadWrite
            }
            else if is_read {
                FilterType::Readable
            }
            else if is_write {
                FilterType::Writable
            }
            else {
                FilterType::UnsupportedFilter(item.events as i16)
            };
            
            let flag_type = if item.events & EPOLLERR != 0 {
                FlagType::Error
            }
            else if item.events & EPOLLHUP != 0 {
                FlagType::Hangup
            }
            else {
                FlagType::Normal
            };
            
            buffer.push(EpollEvent { fd: item.data as i32, filter_type, flag_type });
        }
        
        Ok(num_events as usize)
    }
}

fn create_epoll_event(source: i32, interests: Interests) -> epoll_event {
    match interests {
        Interests::Readable => epoll_event { events: EPOLLIN, data: source as u64 },
        Interests::Writable => epoll_event { events: EPOLLOUT, data: source as u64 },
        Interests::ReadWrite => epoll_event { events: EPOLLIN | EPOLLOUT, data: source as u64 }
    }
}

pub struct EpollEvent {
    fd: i32,
    filter_type: FilterType,
    flag_type: FlagType
}

impl PollEvent for EpollEvent {
    type Source = i32;
    
    fn source(&self) -> Self::Source {
        self.fd
    }
    
    fn filter_type(&self) -> FilterType {
        self.filter_type
    }
    
    fn flag_type(&self) -> FlagType {
        self.flag_type
    }
}