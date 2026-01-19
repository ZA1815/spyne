use crate::serialization::Encoder;

pub struct BinarySerde {
    buffer: Vec<u8>
}

impl BinarySerde {
    fn new() -> Self {
        Self {
            buffer: Vec::<u8>::new()
        }
    }
}

impl Encoder for BinarySerde {
    fn write_u8(&mut self, n: u8) {
        self.buffer.push(n);
    }
    
    fn write_u16(&mut self, n: u16) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_u32(&mut self, n: u32) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_u64(&mut self, n: u64) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_u128(&mut self, n: u128) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_i8(&mut self, n: i8) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_i16(&mut self, n: i16) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_i32(&mut self, n: i32) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_i64(&mut self, n: i64) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_i128(&mut self, n: i128) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_f32(&mut self, n: f32) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_f64(&mut self, n: f64) {
        self.buffer.extend_from_slice(&n.to_le_bytes());
    }
    
    fn write_bool(&mut self, b: bool) {
        self.buffer.extend_from_slice(&(b as u8).to_le_bytes());
    }
    
    fn write_bytes(&mut self, bytes: &[u8]) {
       self.buffer.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
       self.buffer.extend_from_slice(bytes);
    }
    
    fn write_string(&mut self, s: &str) {
       self.buffer.extend_from_slice(&(s.len() as u64).to_le_bytes());
       self.buffer.extend_from_slice(s.as_bytes());
    }
    
    fn write_seq<F>(&mut self, len: usize, f: F)
    where F: FnOnce(&mut Self) {
        self.buffer.extend_from_slice(&(len as u64).to_le_bytes());
        f(self)
    }
    
    fn write_struct<F>(&mut self, _name: &str, fields: &[&str], f: F)
    where F: FnOnce(&mut Self) {
        f(self)
    }
    
    fn write_tuple<F>(&mut self, _len: usize, f: F)
    where F: FnOnce(&mut Self) {
        f(self)
    }
    
    fn write_enum<F>(&mut self, _enum_name: &str, variant_index: u32, _variant_name: &str, f: F)
    where F: FnOnce(&mut Self) {
        self.buffer.extend_from_slice(&variant_index.to_le_bytes());
        f(self)
    }
}