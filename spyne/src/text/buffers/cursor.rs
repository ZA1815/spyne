use std::marker::PhantomData;

use crate::text::buffers::TextBuffer;

pub struct Cursor<T: TextBuffer> {
    buffer: PhantomData<T>,
    position: usize
}

impl<T: TextBuffer> Cursor<T> {
    pub fn new() -> Self {
        Self {
            buffer: PhantomData,
            position: 0
        }
    }
    
    pub fn position(&self) -> usize {
        self.position
    }
    
    pub fn move_left(&mut self, offset: usize) -> usize {
        if self.position.checked_sub(offset) != None {
            self.position -= offset;
        }
        
        self.position
    }
    
    pub fn move_right(&mut self, offset: usize, len: usize) -> usize {
        if self.position + offset <= len {
            self.position += offset;
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
    
    pub fn move_to_next_word_end(&mut self, buffer: &impl TextBuffer) { 
        let line_to_end = buffer.read(self.position, buffer.len());
        let mut in_word = false;
        let mut offset = buffer.len() - self.position;
        for (i, char) in line_to_end.enumerate() {
            if char.is_alphanumeric() {
               in_word = true;
               continue;
            }
            if in_word {
                offset = i;
                break;
            }
        }
        
        self.move_right(offset, buffer.len());
    }
    
    pub fn move_to_prev_word_start(&mut self, buffer: &impl TextBuffer) {
        let line_from_start = buffer.read(0, self.position);
        let mut in_word = false;
        let mut offset = self.position;
        for (i, char) in line_from_start.rev().enumerate() {
            if char.is_alphanumeric() {
                in_word = true;
                continue;
            }
            if in_word {
                offset = i;
                break;
            }
        }
        
        self.move_left(offset);
    }
    
    pub fn delete_prev_word(&mut self, buffer: &mut impl TextBuffer) {
        let line_from_start = buffer.read(0, self.position);
        let mut in_word = false;
        let mut offset = self.position;
        for (i, char) in line_from_start.rev().enumerate() {
            if *char != ' ' {
                in_word = true;
                continue;
            }
            if in_word {
                offset = i;
                break;
            }
        }
        
        self.move_left(offset);
        buffer.delete(self.position, offset);
    }
    
    pub fn delete_next_word(&mut self, buffer: &mut impl TextBuffer) {
        let line_to_end = buffer.read(self.position, buffer.len());
        let mut in_word = false;
        let mut offset = buffer.len() - self.position;
        for (i, char) in line_to_end.enumerate() {
            if *char != ' ' {
                in_word = true;
                continue;
            }
            if in_word {
                offset = i;
                break;
            }
        }
        
        buffer.delete(self.position, offset);
    }
    
    pub fn delete_to_line_start(&mut self, buffer: &mut impl TextBuffer) {
        buffer.delete(0, self.position);
        self.position = 0;
    }
    
    pub fn delete_to_line_end(&self, buffer: &mut impl TextBuffer) {
        buffer.delete(self.position, buffer.len() - self.position);
    }
    
    // Implement yanking (kill) functions later
}