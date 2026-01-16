use crate::text::TextBuffer;

pub struct Cursor {
    position: usize
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: 0
        }
    }
    
    pub fn position(&self) -> usize {
        self.position
    }
    
    pub fn move_left(&mut self) -> usize {
        if self.position != 0 {
            self.position -= 1;
        }
        
        self.position
    }
    
    pub fn move_right(&mut self, len: usize) -> usize {
        if self.position != len {
            self.position += 1;
        }
        
        self.position
    }
    
    pub fn home(&mut self) {
        self.position = 0;
    }
    
    pub fn end(&mut self, len: usize) {
        self.position = len;
    }
    
    pub fn backspace(&mut self, buffer: &mut impl TextBuffer) {
        if self.position != 0 {
            self.position -= 1;
            buffer.delete(self.position, 1);
        }
    }
    
    pub fn delete(&self, buffer: &mut impl TextBuffer) {
        if self.position != buffer.len() {
            buffer.delete(self.position, 1);
        }
    }
}