// #[cfg(feature = "text-gap-buffer")]
// Remove this later when I actually publish
mod gap_buffer;
pub use gap_buffer::GapBuffer;

pub trait TextBuffer {
    fn create_buffer(gap_size: usize) -> Self where Self: Sized;
    fn insert(&mut self, pos: usize, char: char); 
    fn delete(&mut self, start: usize, len: usize);
    fn read(&self) -> impl Iterator<Item = &char>;
    fn len(&self) -> usize;
}