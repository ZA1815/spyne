use crate::serialization::{Decoder, Encoder};

pub struct BinarySerde {
    buffer: Vec<u8>,
    read_index: usize
}

impl BinarySerde {
    fn new(buffer: Option<Vec<u8>>) -> Self {
        Self {
            buffer: match buffer {
                Some(buf) => buf,
                None => Vec::new()
            },
            read_index: 0
        }
    }
    
    fn read_inc<const N: usize>(&mut self) -> Result<[u8; N], String> {
        if self.read_index + N > self.buffer.len() {
            return Err("BinarySerde: Deserialization out of bounds.".to_string());
        }
        
        let slice = self.buffer[self.read_index..self.read_index + N].try_into().unwrap();
        self.read_index += N;
        Ok(slice)
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
    
    fn write_struct<F>(&mut self, _name: &str, _fields: &[&str], f: F)
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

impl Decoder for BinarySerde {
    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(u8::from_le_bytes(self.read_inc::<1>()?))
    }
    
    fn read_u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.read_inc::<2>()?))
    }
    
    fn read_u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.read_inc::<4>()?))
    }
    
    fn read_u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.read_inc::<8>()?))
    }
    
    fn read_u128(&mut self) -> Result<u128, String> {
        Ok(u128::from_le_bytes(self.read_inc::<16>()?))
    }
    
    fn read_i8(&mut self) -> Result<i8, String> {
        Ok(i8::from_le_bytes(self.read_inc::<1>()?))
    }
    
    fn read_i16(&mut self) -> Result<i16, String> {
        Ok(i16::from_le_bytes(self.read_inc::<2>()?))
    }
    
    fn read_i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(self.read_inc::<4>()?))
    }
    
    fn read_i64(&mut self) -> Result<i64, String> {
        Ok(i64::from_le_bytes(self.read_inc::<8>()?))
    }
    
    fn read_i128(&mut self) -> Result<i128, String> {
        Ok(i128::from_le_bytes(self.read_inc::<16>()?))
    }
    
    fn read_f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_le_bytes(self.read_inc::<4>()?))
    }
    
    fn read_f64(&mut self) -> Result<f64, String> {
        Ok(f64::from_le_bytes(self.read_inc::<8>()?))
    }
    
    fn read_bool(&mut self) -> Result<bool, String> {
        Ok(u8::from_le_bytes(self.read_inc::<1>()?) != 0)
    }
    
    fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        let len = u64::from_le_bytes(self.read_inc::<8>()?) as usize;
        if self.read_index + len > self.buffer.len() {
            return Err("BinarySerde: Buffer underflow while deserializing bytes.".to_string());
        }
        
        let vec = self.buffer[self.read_index..self.read_index + len].to_vec();
        self.read_index += len;
        
        Ok(vec)
    }
    
    fn read_string(&mut self) -> Result<String, String> {
        let len = u64::from_le_bytes(self.read_inc::<8>()?) as usize;
        if self.read_index + len > self.buffer.len() {
            return Err("BinarySerde: Buffer underflow while deserializing String.".to_string());
        }
        
        let vec = self.buffer[self.read_index..self.read_index + len].to_vec();
        self.read_index += len;
        
        Ok(
            String::from_utf8(vec)
                .map_err(|_| "BinarySerde: Invalid UTF-8 characters while deserializing String".to_string())?
        )
    }
    
    fn read_seq<F, T>(&mut self, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, usize) -> Result<T, String> {
        let len = u64::from_le_bytes(self.read_inc::<8>()?) as usize;
        
        Ok(f(self, len)?)
    }
    
    fn read_tuple<F, T>(&mut self, _len: usize, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String> {
        f(self)
    }
    
    fn read_struct<F, T>(&mut self, _name: &str, _fields: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String> {
        f(self)
    }
    
    fn read_enum<F, T>(&mut self, _enum_name: &str, _variants: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, u32) -> Result<T, String> {
        let var_index = u32::from_le_bytes(self.read_inc::<4>()?);
        f(self, var_index)
    }
}