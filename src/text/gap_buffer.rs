use crate::text::TextBuffer;

pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize
}

impl TextBuffer for GapBuffer {
    fn create_buffer(gap_size: usize) -> Self where Self: Sized {
        Self { buffer: vec![char::default(); gap_size], gap_start: 0, gap_end: gap_size }
    }
    
    fn insert(&mut self, pos: usize, char: char) {
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
    
    fn delete(&mut self, start: usize, len: usize) {
        self.move_gap(start);
        if self.gap_end + len > self.buffer.len() {
            panic!("GapBuffer: Delete range out of bounds.");
        }
        self.gap_end += len;
    }
    
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
    
    fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }
}

impl GapBuffer {
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