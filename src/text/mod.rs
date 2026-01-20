#[cfg(feature = "text-gapbuffer")]
mod gap_buffer;
#[cfg(feature = "text-gapbuffer")]
pub use gap_buffer::GapBuffer;

#[cfg(feature = "text-cursor")]
mod cursor;
#[cfg(feature = "text-cursor")]
pub use cursor::Cursor;

pub trait TextBuffer {
    fn create_buffer(gap_size: usize) -> Self where Self: Sized;
    fn insert(&mut self, pos: usize, char: char); 
    fn delete(&mut self, start: usize, len: usize);
    fn read(&self, start: usize, end: usize) -> impl DoubleEndedIterator<Item = &char>;
    fn len(&self) -> usize;
}