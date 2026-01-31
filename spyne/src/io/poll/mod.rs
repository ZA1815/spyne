// #[cfg(feature = "io-poll-epoll")]
mod epoll;
// #[cfg(feature = "io-poll-epoll")]
pub use epoll::Epoll;

// #[cfg(feature = "io-poll-kqueue")]
mod kqueue;
// #[cfg(feature = "io-poll-kqueue")]
pub use kqueue::Kqueue;

use std::time::Duration;

pub trait Poller {
    type Source: Copy;
    type Event: PollEvent<Source = Self::Source>;
    fn add(&mut self, source: Self::Source, interests: Interests);
    fn modify(&mut self, source: Self::Source, interests: Interests);
    fn remove(&mut self, source: Self::Source);
    fn wait(&mut self, timeout: Duration, buffer: &mut Vec<Self::Event>, max_events: i32) -> Result<usize, PollError>;
}

pub trait PollEvent {
    type Source: Copy;
    fn source(&self) -> Self::Source;
    fn filter_type(&self) -> FilterType;
    fn flag_type(&self) -> FlagType;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Interests {
    Readable,
    Writable,
    ReadWrite
}

#[derive(Clone, Copy)]
pub enum FilterType {
    Readable,
    Writable,
    ReadWrite,
    UnsupportedFilter(i16)
}

#[derive(Clone, Copy)]
pub enum FlagType {
    Normal,
    Error,
    Hangup
}

pub enum PollError {
    Interrupt,
    InvalidFd,
    InvalidArgument,
    OutOfMemory,
    Unknown(i32)
}