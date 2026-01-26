use crate::primitives::text::TextBuffer;

/// A gap buffer structure.
/// 
/// Useful for small and frequent insertions and deletions.
/// O(1) insertions and deletions on average.
/// O(n) gap repositioning on edit location change.
/// 
/// Example use case: Shell
pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize
}

impl TextBuffer for GapBuffer {
    /// Creates a gap buffer according to gap size.
    /// 
    /// Start is at first insertable location.
    /// End is at first character after last insertable location.
    /// 
    /// Buffer is initialized to designated gap size.
    fn create_buffer(init_size: usize) -> Self where Self: Sized {
        Self { buffer: vec![char::default(); init_size], gap_start: 0, gap_end: init_size }
    }
    
    /// Inserts into a gap buffer.
    /// 
    /// 1. Moves gap
    /// 2. Checks if gap is empty (if so, resize by doubling existing buffer length)
    /// 3. Insert at `gap_start` and increment gap_start by 1 (slot filled)
    /// 
    /// # Panics
    /// Panics if `pos` is out of bounds.
    fn insert(&mut self, pos: usize, char: char) {
        if pos >= self.len() {
            panic!("GapBuffer: Insert range out of bounds ({} >= {})", pos, self.len());
        }
        self.move_gap(pos);
        
        if self.gap_start == self.gap_end {
            let old_len = self.buffer.len();
            self.buffer.resize(old_len * 2, char::default());
            let new_len = self.buffer.len(); 
            let range_size = old_len - self.gap_start;
            self.buffer.copy_within(self.gap_start..old_len, new_len - range_size);
            self.gap_end = new_len - range_size;
        }
        
        self.buffer[self.gap_start] = char;
        self.gap_start += 1;
    }
    
    /// Deletes from a gap buffer.
    /// 
    /// 1. Moves gap
    /// 2. Extends gap by `len` by adding to `gap_end`
    /// 
    /// # Panics
    /// Panics if `gap_end` + `len` is out of bounds.
    fn delete(&mut self, start: usize, len: usize) {
        self.move_gap(start);
        if self.gap_end + len > self.buffer.len() {
            panic!("GapBuffer: Delete range out of bounds.");
        }
        self.gap_end += len;
    }
    
    /// Reads from a gap buffer.
    /// 
    /// Reads from the left and right of the buffer, skipping over the gap.
    /// 
    /// # Panics
    /// Panics if `start` is out of bounds.
    /// Panics if `end` is out of bounds.
    /// Panics if `start` is greater than `end`.
    fn read(&self, start: usize, end: usize) -> impl DoubleEndedIterator<Item = &char> {
        let len = self.buffer.len();
        if start >= len {
            panic!("GapBuffer: Read range out of bounds ({} >= {}).", start, len);
        }
        else if end > len {
            panic!("GapBuffer: Read range out of bounds ({} > {}).", end, len)
        }
        else if start > end {
            panic!("GapBuffer: Read range invalid ({} > {}).", start, end);
        }
        
        let left = &self.buffer[start..self.gap_start];
        let right = &self.buffer[self.gap_end..end];
        
        left.iter().chain(right.iter())
    }
    
    /// Calculates the length of a gap buffer.
    /// 
    /// Length = Length of buffer - size of gap
    fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }
}

impl GapBuffer {
    /// Helper method to move the gap.
    /// 
    /// First determine direction to move in.
    /// 
    /// LEFT:
    /// 1. Determine distance between `pos` and `gap_start`.
    /// 2. Copy all characters between `pos` and `gap_start` inclusive to the other side of the gap.
    /// 3. Move gap to the left by subtracting `dist` from gap bounds.
    /// 
    /// LEFT EXAMPLE:
    /// [H, E (pos), L, _ (gap_start), _, _, _, _, L (gap_end), O], `pos` = 1, `gap_start` = 3, `gap_end` = 8
    /// 
    /// 1. dist = 2
    /// 
    /// 2. Characters between `pos` and `gap_start`: E, L
    /// [H, E (pos), L, _ (gap_start), _, _, E, L, L (gap_end), O]
    /// 
    /// 3. "Retrieve" consumed gap spaces:
    /// [H, E (gap_start/pos), L, _, _, _, E, L, L (gap_end), O]
    /// 
    /// "Final" buffer: [H, _ (gap_start), _, _, _, _, E (gap_end), L, L, O]
    /// 
    /// RIGHT:
    /// 1. Determine "real position" because the user of the API doesn't account for gap indices
    /// 2. Determine distance between `pos` and `gap_start` to calculate the distance the gap has to move
    /// 3. Copy all characters between `gap_end` and the "real position" to the other side of the gap.
    /// 4. Move gap to the right by adding `dist` to gap bounds.
    /// 
    /// RIGHT EXAMPLE:
    /// [H, _ (gap_start), _, _, _, _, E (gap_end), L, L (pos), O], `pos` = 3, `gap_start` = 1, `gap_end` = 6
    /// 
    /// 1. real_pos = 8
    /// 
    /// 2. dist = 2
    /// 
    /// 3. Characters between `gap_end` and `real_pos`: E, L
    /// [H, E (gap_start), L, _, _, _, E (gap_end), L, L (pos), O]
    /// 
    /// 4. "Retrieve" consumed gap spaces:
    /// [H, E, L, _ (gap_start), _, _, E, L, L (gap_end/pos), O]
    /// 
    /// "Final" buffer: [H, E, L, _ (gap_start), _, _, _, _, L (gap_end/pos), O]
    fn move_gap(&mut self, pos: usize) {
        if self.gap_start > pos {
            let dist = self.gap_start - pos;
            self.buffer.copy_within(pos..self.gap_start, self.gap_end - dist);
            self.gap_start -= dist;
            self.gap_end -= dist;
        }
        else if self.gap_start < pos {
            let real_pos = pos + self.gap_end - self.gap_start;
            let dist = pos - self.gap_start;
            self.buffer.copy_within(self.gap_end..real_pos, self.gap_start);
            self.gap_start += dist;
            self.gap_end += dist;
        }
    }
}