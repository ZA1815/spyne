use crate::serialization::serialize::Serializer;

pub struct JsonSerde {
    buffer: String,
    stack: Vec<Group>
}

struct Group {
    delimiter: Delimiter,
    item_placed: bool,
    is_key: bool
}

#[derive(PartialEq, Eq)]
enum Delimiter {
    Object,
    Array
}

impl JsonSerde {
    fn format_element(&mut self) {
        let group = self.stack.last_mut().unwrap();
        if group.item_placed {
            if group.delimiter == Delimiter::Object {
                if group.is_key {
                    self.buffer.push(',');
                    group.is_key = false;
                }
                else {
                    self.buffer.push(':');
                    group.is_key = true;
                }
            }
        }
        else {
            group.item_placed = true;
        }
    }
}

impl Serializer for JsonSerde {
    fn write_u8(&mut self, n: u8) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_u16(&mut self, n: u16) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_u32(&mut self, n: u32) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_u64(&mut self, n: u64) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_u128(&mut self, n: u128) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_i8(&mut self, n: i8) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_i16(&mut self, n: i16) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_i32(&mut self, n: i32) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_i64(&mut self, n: i64) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_i128(&mut self, n: i128) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_f32(&mut self, n: f32) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_f64(&mut self, n: f64) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", n.to_string()));
        }
        else {
            self.buffer.push_str(&n.to_string());
        }
    }
    
    fn write_bool(&mut self, b: bool) {
        self.format_element();
        if self.stack.last().unwrap().is_key {
            self.buffer.push_str(&format!("\"{}\"", b.to_string()));
        }
        else {
            self.buffer.push_str(&b.to_string());
        }
    }
    
    fn write_bytes(&mut self, bytes: &[u8]) {
        // TODO: Base64 encoding
    }
    
    // Edge case: if s contains " or \, they need to be escaped
    fn write_string(&mut self, s: &str) {
        self.format_element();
        self.buffer.push_str(&format!("\"{}\"", s));
    }
    
    fn write_seq<F>(&mut self, _len: usize, f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a sequence as a key");
            }
        }
        self.buffer.push('[');
        self.stack.push(Group {
            delimiter: Delimiter::Array,
            item_placed: false,
            is_key: false
        });
        f(self);
        self.stack.pop();
        self.buffer.push(']');
    }
    
    fn write_map<F>(&mut self, _len: usize, f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a map as a key");
            }
        }
        self.buffer.push('{');
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: false,
            is_key: true
        });
        f(self);
        self.stack.pop();
        self.buffer.push('}');
    }
    
    fn write_tuple<F>(&mut self, _len: usize, f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a tuple as a key");
            }
        }
        self.buffer.push('[');
        self.stack.push(Group {
            delimiter: Delimiter::Array,
            item_placed: false,
            is_key: false
        });
        f(self);
        self.stack.pop();
        self.buffer.push(']');
    }
    
    fn write_struct<F>(&mut self, _name: &str, _fields: &[&str], f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a struct as a key");
            }
        }
        self.buffer.push('{');
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: false,
            is_key: true
        });
        f(self);
        self.stack.pop();
        self.buffer.push('}');
    }
    
    fn write_enum<F>(&mut self, _enum_name: &str, _variant_index: u32, variant_name: &str, f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use an enum as a key");
            }
        }
        self.buffer.push('{');
        self.buffer.push_str(&format!("\"{}\"", variant_name));
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: true,
            is_key: false
        });
        f(self);
        self.stack.pop();
        self.buffer.push('}');
    }
}