use std::time::Duration;

pub trait Poller {
    type Source: Copy;
    type Event: PollEvent<Source = Self::Source>;
    fn add(&mut self, source: Self::Source, interests: Interests);
    fn modify(&mut self, source: Self::Source, interests: Interests);
    fn remove(&mut self, source: Self::Source);
    fn wait(&mut self, timeout: Duration, buffer: &mut Vec<Self::Event>);
}

pub trait PollEvent {
    type Source: Copy;
    fn source(&self) -> Self::Source;
    fn event_type(&self) -> EventType;
}

pub enum Interests {
    R,
    W,
    RW
}

pub enum EventType {
    Readable,
    Writable,
    Error,
    Hangup
}