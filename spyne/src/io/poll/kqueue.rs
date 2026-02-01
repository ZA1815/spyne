use std::{collections::HashMap, ffi::c_void, ptr::{null, null_mut}, time::Duration};

use spyne_ffi::c::macos::{general::syscalls::__error, kqueue::{constants::{EV_ADD, EV_DELETE, EV_EOF, EV_ERROR, EVFILT_READ, EVFILT_WRITE}, syscalls::{kevent, kqueue}, types::{kevent, timespec}}};

use crate::io::poll::{FilterType, FlagType, Interests, PollError, PollEvent, Poller};

pub struct Kqueue {
    fd: i32,
    registered: HashMap<i32, Interests>
}

impl Kqueue {
    pub fn new() -> Self {
        Self {
            fd: unsafe { kqueue() },
            registered: HashMap::new()
        }
    }
    
    fn interest_wanted(&mut self, source: i32, old_interests: Interests, new_interests: Interests) {
        match old_interests {
            Interests::Readable if new_interests == Interests::ReadWrite => {
                self.add(source, Interests::Writable);
            }
            Interests::Writable if new_interests == Interests::ReadWrite => {
                self.add(source, Interests::Readable);
            }
            _ => unreachable!()
        }
    }
    
    fn interest_unwanted(&mut self, source: i32, old_interests: Interests, new_interests: Interests) {
        match old_interests {
            Interests::ReadWrite if new_interests == Interests::Readable => {
                let event = create_kevent(source, EVFILT_READ, EV_DELETE);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            Interests::ReadWrite if new_interests == Interests::Writable => {
                let event = create_kevent(source, EVFILT_WRITE, EV_DELETE);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            _ => unreachable!()
        }
    }
    
    fn interest_wanted_unwanted(&mut self, source: i32, old_interests: Interests, new_interests: Interests) {
        match old_interests {
            Interests::Readable if new_interests == Interests::Writable => {
                let events = [
                    create_kevent(source, EVFILT_READ, EV_DELETE),
                    create_kevent(source, EVFILT_WRITE, EV_ADD)
                ];
                let res = unsafe { kevent(self.fd, events.as_ptr(), 2, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            Interests::Writable if new_interests == Interests::Readable => {
                let events = [
                    create_kevent(source, EVFILT_WRITE, EV_DELETE),
                    create_kevent(source, EVFILT_READ, EV_ADD)
                ];
                let res = unsafe { kevent(self.fd, events.as_ptr(), 2, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            _ => unreachable!()
        }
    }
}

impl Poller for Kqueue {
    type Source = i32;
    type Event = KqueueEvent;
    
    fn add(&mut self, source: Self::Source, interests: Interests) {
        match interests {
            Interests::Readable => {
                let event = create_kevent(source, EVFILT_READ, EV_ADD);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            Interests::Writable => {
                let event = create_kevent(source, EVFILT_WRITE, EV_ADD);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
            Interests::ReadWrite => {
                let events = [
                    create_kevent(source, EVFILT_READ, EV_ADD),
                    create_kevent(source, EVFILT_WRITE, EV_ADD)
                ];
                let res = unsafe { kevent(self.fd, events.as_ptr(), 2, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
        }
        
        self.registered.insert(source, interests);
    }
    
    fn modify(&mut self, source: Self::Source, new_interests: Interests) {
        match self.registered.get(&source) {
            Some(old_interests) => {
                if *old_interests == new_interests {
                    return;
                }
                
                match old_interests {
                    Interests::ReadWrite => self.interest_unwanted(source, *old_interests, new_interests),
                    _ => {
                        if *old_interests == Interests::Readable && new_interests == Interests::Writable || *old_interests == Interests::Writable && new_interests == Interests::Readable {
                            self.interest_wanted_unwanted(source, *old_interests, new_interests);
                        }
                        else {
                            self.interest_wanted(source, *old_interests, new_interests);
                        }
                    }
                }
            }
            None => self.add(source, new_interests),
        }
        
        self.registered.insert(source, new_interests);
    }
    
    fn remove(&mut self, source: Self::Source) {
        let interests = match self.registered.get(&source) {
            Some(interests) => interests,
            None => return
        };
        match interests {
            Interests::Readable => {
                let event = create_kevent(source, EVFILT_READ, EV_DELETE);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            },
            Interests::Writable => {
                let event = create_kevent(source, EVFILT_WRITE, EV_DELETE);
                let res = unsafe { kevent(self.fd, &event, 1, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            },
            Interests::ReadWrite => {
                let events = [
                    create_kevent(source, EVFILT_READ, EV_DELETE),
                    create_kevent(source, EVFILT_WRITE, EV_DELETE)
                ];
                let res = unsafe { kevent(self.fd, events.as_ptr(), 2, null_mut(), 0, null()) };
                if res < 0 {
                    // Add error later
                }
            }
        };
        
        self.registered.remove(&source);
    }
    
    fn wait(&mut self, timeout: Duration, buffer: &mut Vec<Self::Event>, max_events: i32) -> Result<usize, PollError> {
        let mut kevent_buffer = Vec::<kevent>::with_capacity(max_events as usize);
        let duration_to_timespec = timespec {
            tv_sec: timeout.as_secs() as i64,
            tv_nsec: timeout.subsec_nanos() as i64
        };
        let num_events = unsafe { kevent(self.fd, null(), 0, kevent_buffer.as_mut_ptr(), max_events, &duration_to_timespec) };
        if num_events < 0 {
            let errno: i32 = unsafe { *__error() };
            match errno {
                4 => return Err(PollError::Interrupt),
                9 => return Err(PollError::InvalidFd),
                12 => return Err(PollError::OutOfMemory),
                22 => return Err(PollError::InvalidArgument),
                _ => return Err(PollError::Unknown(errno))
            }
        }
        unsafe { kevent_buffer.set_len(num_events as usize) };
        buffer.clear();
        for item in kevent_buffer {
            let filter_type = match item.filter {
                EVFILT_READ => FilterType::Readable,
                EVFILT_WRITE => FilterType::Writable,
                _ => FilterType::UnsupportedFilter(item.filter)
            };
            let flag_type = if item.flags & (EV_ERROR | EV_EOF) != 0 {
                FlagType::Error // Add bitflags later so we can represent both
            }
            else if item.flags & EV_ERROR != 0 {
                FlagType::Error
            }
            else if item.flags & EV_EOF != 0 {
                FlagType::Hangup
            }
            else {
                FlagType::Normal
            };
            
            buffer.push(KqueueEvent {
                fd: item.ident as i32, filter_type, flag_type
            });
        }
        Ok(num_events as usize)
    }
}

fn create_kevent(source: i32, filter: i16, flags: u16) -> kevent {
    kevent {
        ident: source as usize,
        filter,
        flags,
        fflags: 0,
        data: 0,
        udata: 0 as *mut c_void
    }
}

pub struct KqueueEvent {
    fd: i32,
    filter_type: FilterType,
    flag_type: FlagType
}

impl PollEvent for KqueueEvent {
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