// #[cfg(feature = "primitives-text-gapbuffer")]
mod gap_buffer;
// #[cfg(feature = "primitives-text-gapbuffer")]
pub use gap_buffer::GapBuffer;

// #[cfg(feature = "primitives-text-cursor")]
mod cursor;
// #[cfg(feature = "primitives-text-cursor")]
pub use cursor::Cursor;

/// A trait for creating and manipulating text buffers.
/// 
/// Current implementations: [`GapBuffer`]
/// Planned implementations: Rope, PieceTable
pub trait TextBuffer {
    /// Creates a new text buffer.
    /// 
    /// `init_size` parameter determines the initial size of the buffer
    fn create_buffer(init_size: usize) -> Self where Self: Sized;
    /// Inserts into a text buffer.
    /// 
    /// Inserts `char` at `pos` (0-indexed).
    /// Reallocates buffer if filled.
    /// 
    /// # Panics
    /// 
    /// Panics if `pos` is out of bounds.
    fn insert(&mut self, pos: usize, char: char); 
    /// Deletes from a text buffer.
    /// 
    /// Deletes `len` characters starting at `start` (0-indexed)
    /// 
    /// # Panics
    /// 
    /// To be filled in later.
    fn delete(&mut self, start: usize, len: usize);
    /// Reads from a text buffer.
    /// 
    /// Returns a `DoubleEndedIterator` for bidirectional traversal.
    /// Reads characters from start (0-indexed) to end (exclusive).
    /// 
    /// # Panics
    /// 
    /// Panics if `start` is out of bounds.
    /// Panics if `end` is out of bounds.
    /// Panics if `start` is greater than `end`.
    fn read(&self, start: usize, end: usize) -> impl DoubleEndedIterator<Item = &char>;
    /// Returns the current length of a text buffer.
    fn len(&self) -> usize;
}