// TODO: Remove this later when I actually publish
// #[cfg(feature = "text-gap-buffer")]
mod gap_buffer;
pub use gap_buffer::GapBuffer;

// #[cfg(feature = "text-cursor")]
mod cursor;
pub use cursor::Cursor;

pub trait TextBuffer {
    fn create_buffer(gap_size: usize) -> Self where Self: Sized;
    fn insert(&mut self, pos: usize, char: char); 
    fn delete(&mut self, start: usize, len: usize);
    fn read(&self, start: usize, end: usize) -> impl DoubleEndedIterator<Item = &char>;
    fn len(&self) -> usize;
}