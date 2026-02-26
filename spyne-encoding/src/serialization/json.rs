use crate::{serialization::{deserialize::{Deserialize, Deserializer}, serialize::{Serialize, Serializer}}, transfer::base64::{decode, encode_into}};

pub struct JsonSerde<'a> {
    buffer: String,
    stack: Vec<Group<'a>>,
    read_index: usize
}

#[derive(Default)]
struct Group<'a> {
    delimiter: Delimiter,
    item_placed: bool,
    is_key: bool,
    is_struct: bool,
    keys: &'a [&'a str],
    current_key: usize
}

#[derive(PartialEq, Eq)]
enum Delimiter {
    Object,
    Array
}

impl Default for Delimiter {
    fn default() -> Self {
        Self::Array
    }
}

impl<'a> JsonSerde<'a> {
    fn new(buffer: Option<String>) -> Self {
        Self {
            buffer: match buffer {
                Some(buf) => buf,
                None => String::new()
            },
            stack: Vec::new(),
            read_index: 0
        }
    }
        
    fn read_prim(&mut self) -> Result<&str, String> {
        self.skip_whitespace();
        self.skip_field_name();
        
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ':' {
                self.read_index += 1;
            }
        }
        
        let mut read_to: usize = 0;
        for c in self.buffer.chars().skip(self.read_index) {
            if c == ',' || c == ']' || c == '}' {
                break;
            }
            else {
                read_to += 1;
            }
        }
        
        if self.buffer.len() < self.read_index + read_to {
            return Err("JsonSerde: Deserialization out of bounds".to_string());
        }
        let slice = &self.buffer[self.read_index..self.read_index + read_to];
        self.read_index += read_to;
        
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        
        Ok(slice)
    }
    
    fn count_items(&self) -> usize {
        let mut count: usize = 0;
        let mut depth: usize = 0;
        let mut character_escaped = false;
        let mut in_string = false;
        let mut has_items = false;
        for c in self.buffer.chars().skip(self.read_index) {
            if c == '\\' {
                character_escaped = true;
            }
            else if !character_escaped && c == '"' {
                character_escaped = false;
                in_string = !in_string;
            }
            else if c == '[' || c == '{' {
                if depth == 1 {
                    has_items = true;
                }
                depth += 1;
            }
            else if depth == 0 {
                break;
            }
            else if c == ']' || c == '}' {
                depth -= 1;
            }
            else if depth == 1 && !in_string && c == ',' {
                count += 1;
            }
            else if depth == 1 && c != ' ' {
                has_items = true;
            }
            
        }
        if has_items || count > 0 {
            count += 1;
        }
        
        count
    }
    
    fn skip_whitespace(&mut self) {
        for c in self.buffer.chars().skip(self.read_index) {
            if c == ' ' {
                self.read_index += 1;
            }
            else {
                break;
            }
        }
    }
    
    fn skip_field_name(&mut self) {
        if let Some(group) = self.stack.last() {
            if group.is_struct {
                let mut character_escaped = false;
                self.read_index += 1;
                for c in self.buffer.chars().skip(self.read_index) {
                    if c == '\\' {
                        self.read_index += 1;
                        character_escaped = true;
                        continue;
                    }
                    else if !character_escaped && c == '"' {
                        self.read_index += 1;
                        break;
                    }
                    else {
                        self.read_index += 1;
                        character_escaped = false;
                    }
                }
                self.skip_whitespace();
                self.read_index += 1;
            }
        }
    }
    
    fn format_element(&mut self) {
        if let Some(group) = self.stack.last_mut() {
            if group.is_struct {
                if group.item_placed {
                    self.buffer.push_str(&format!(",\"{}\":", group.keys[group.current_key]));
                }
                else {
                    self.buffer.push_str(&format!("\"{}\":", group.keys[group.current_key]));
                    group.item_placed = true;
                }
                group.current_key += 1;
            }
            else if group.item_placed {
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
                else if group.delimiter == Delimiter::Array {
                    self.buffer.push(',');
                }
            }
            else {
                group.item_placed = true;
            }
        }
    }
    
    pub fn serialize<T: Serialize<'a>>(item: &T) -> Result<String, String> {
        let mut serde = JsonSerde::new(None);
        item.serialize(&mut serde);
        Ok(serde.buffer.clone())
    }
    
    pub fn deserialize<T: Deserialize>(buffer: String) -> Result<T, String> {
        let mut serde = JsonSerde::new(Some(buffer));
        Ok(T::deserialize(&mut serde)?)
    }
}

impl<'a> Serializer<'a> for JsonSerde<'a> {
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
        self.format_element();
        self.buffer.push('"');
        encode_into(bytes, &mut self.buffer);
        self.buffer.push('"');
    }
    
    fn write_string(&mut self, s: &str) {
        self.format_element();
        self.buffer.push('"');
        for c in s.chars() {
            if c == '"' || c == '\\' {
                self.buffer.push('\\');
                self.buffer.push(c);
            }
            else {
                self.buffer.push(c);
            }
        }
        self.buffer.push('"');
    }
    
    fn write_seq<F>(&mut self, _len: usize, f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a sequence as a key");
            }
        }
        self.format_element();
        self.buffer.push('[');
        self.stack.push(Group {
            delimiter: Delimiter::Array,
            item_placed: false,
            is_key: false,
            is_struct: false,
            keys: &[],
            current_key: 0
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
        self.format_element();
        self.buffer.push('{');
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: false,
            is_key: true,
            is_struct: false,
            keys: &[],
            current_key: 0
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
        self.format_element();
        self.buffer.push('[');
        self.stack.push(Group {
            delimiter: Delimiter::Array,
            item_placed: false,
            is_key: false,
            is_struct: false,
            keys: &[],
            current_key: 0
        });
        f(self);
        self.stack.pop();
        self.buffer.push(']');
    }
    
    fn write_struct<F>(&mut self, _name: &str, fields: &'a [&'a str], f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use a struct as a key");
            }
        }
        self.format_element();
        self.buffer.push('{');
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: false,
            is_key: false,
            is_struct: true,
            keys: fields,
            current_key: 0
        });
        f(self);
        self.stack.pop();
        self.buffer.push('}');
    }
    
    fn write_enum<F>(&mut self, _enum_name: &str, _variant_index: u32, variant_name: &'a [&'a str], f: F)
    where F: FnOnce(&mut Self) {
        if let Some(group) = self.stack.last() {
            if group.delimiter == Delimiter::Object && group.is_key {
                panic!("JsonSerde: JSON keys must be primitives or strings, tried to use an enum as a key");
            }
        }
        self.format_element();
        self.buffer.push('{');
        self.buffer.push_str(&format!("\"{}\"", variant_name[0]));
        self.stack.push(Group {
            delimiter: Delimiter::Object,
            item_placed: true,
            is_key: false,
            is_struct: false,
            keys: variant_name,
            current_key: 0
        });
        f(self);
        self.stack.pop();
        self.buffer.push('}');
    }
}

impl<'a> Deserializer for JsonSerde<'a> {
    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.read_prim()?.parse::<u8>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_u16(&mut self) -> Result<u16, String> {
        Ok(self.read_prim()?.parse::<u16>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_u32(&mut self) -> Result<u32, String> {
        Ok(self.read_prim()?.parse::<u32>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_u64(&mut self) -> Result<u64, String> {
        Ok(self.read_prim()?.parse::<u64>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_u128(&mut self) -> Result<u128, String> {
        Ok(self.read_prim()?.parse::<u128>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_i8(&mut self) -> Result<i8, String> {
        Ok(self.read_prim()?.parse::<i8>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_i16(&mut self) -> Result<i16, String> {
        Ok(self.read_prim()?.parse::<i16>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_i32(&mut self) -> Result<i32, String> {
        Ok(self.read_prim()?.parse::<i32>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_i64(&mut self) -> Result<i64, String> {
        Ok(self.read_prim()?.parse::<i64>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_i128(&mut self) -> Result<i128, String> {
        Ok(self.read_prim()?.parse::<i128>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_f32(&mut self) -> Result<f32, String> {
        Ok(self.read_prim()?.parse::<f32>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_f64(&mut self) -> Result<f64, String> {
        Ok(self.read_prim()?.parse::<f64>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_bool(&mut self) -> Result<bool, String> {
        Ok(self.read_prim()?.parse::<bool>().map_err(|e| format!("JsonSerde: Deserialization error: {}", e))?)
    }
    
    fn read_bytes(&mut self) -> Result<Vec<u8>, String> {
        Ok(decode(self.read_string()?))
    }
    
    fn read_string(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        self.skip_field_name();
        
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ':' {
                self.read_index += 1;
            }
        }
        
        let mut string = String::new();
        let mut character_escaped = false;
        self.read_index += 1;
        for c in self.buffer.chars().skip(self.read_index) {
            if c == '\\' {
                self.read_index += 1;
                character_escaped = true;
                continue;
            }
            else if !character_escaped && c == '"' {
                self.read_index += 1;
                break;
            }
            else {
                self.read_index += 1;
                character_escaped = false;
                string.push(c);
            }
        }
        
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        
        Ok(string)
    }
    
    fn read_seq<F, T>(&mut self, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, usize) -> Result<T, String> {
        self.skip_whitespace();
        self.skip_field_name();
        self.stack.push(Group::default());
        let item_count = self.count_items();
        self.read_index += 1;
        let seq = f(self, item_count)?;
        self.read_index += 1;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        self.stack.pop();
        
        Ok(seq)
    }
    
    fn read_map<F, T>(&mut self, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, usize) -> Result<T, String> {
        self.skip_whitespace();
        self.skip_field_name();
        self.stack.push(Group::default());
        let item_count = self.count_items();
        self.read_index += 1;
        let map = f(self, item_count)?;
        self.read_index += 1;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        self.stack.pop();
        
        Ok(map)
    }
    
    fn read_tuple<F, T>(&mut self, _len: usize, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String> {
        self.skip_whitespace();
        self.skip_field_name();
        self.stack.push(Group::default());
        self.read_index += 1;
        let seq = f(self)?;
        self.read_index += 1;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        self.stack.pop();
        
        Ok(seq)
    }
    
    fn read_struct<F, T>(&mut self, _name: &str, _fields: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String> {
        self.skip_whitespace();
        self.skip_field_name();
        self.stack.push(Group { is_struct: true, ..Default::default() });
        self.read_index += 1;
        let obj = f(self)?;
        self.read_index += 1;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        self.stack.pop();
        
        Ok(obj)
    }
    
    fn read_enum<F, T>(&mut self, _enum_name: &str, variants: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, u32) -> Result<T, String> {
        self.skip_whitespace();
        self.skip_field_name();
        self.stack.push(Group::default());
        self.read_index += 1;
        let var_name = self.read_string()?;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ':' {
                self.read_index += 1;
            }
        }
        let var = match variants.iter().enumerate().find(|(_, s)| *s == &var_name) {
            Some(var) => var,
            None => return Err("JsonSerde: Variant name not found in possible variants".to_string())
        };
        let obj = f(self, var.0 as u32)?;
        self.read_index += 1;
        if let Some(c) = self.buffer[self.read_index..].chars().next() {
            if c == ',' {
                self.read_index += 1;
            }
        }
        self.stack.pop();
        
        Ok(obj)
    }
}