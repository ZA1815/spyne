// #[cfg(feature = "serialization-binary")]
mod binary;
// #[cfg(feature = "serialization-binary")]
pub use binary::BinarySerde;

use std::{borrow::Cow, collections::{BTreeMap, HashMap, HashSet}, marker::PhantomData};

trait Encode {
    fn encode(&self, encoder: &mut impl Encoder);
}
pub struct Bytes<'a>(pub &'a [u8]);

impl Encode for u8 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u8(*self);
    }
}
impl Encode for u16 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u16(*self);
    }
}
impl Encode for u32 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u32(*self);
    }
}
impl Encode for u64 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u64(*self);
    }
}
impl Encode for u128 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u128(*self);
    }
}
impl Encode for usize {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u64(*self as u64);
    }
}
impl Encode for i8 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i8(*self);
    }
}
impl Encode for i16 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i16(*self);
    }
}
impl Encode for i32 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i32(*self);
    }
}
impl Encode for i64 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i64(*self);
    }
}
impl Encode for i128 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i128(*self);
    }
}
impl Encode for isize {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_i64(*self as i64);
    }
}
impl Encode for f32 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_f32(*self);
    }
}
impl Encode for f64 {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_f64(*self);
    }
}
impl Encode for bool {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_bool(*self);
    }
}
impl Encode for char {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_u32(*self as u32);
    }
}
impl<'a> Encode for Bytes<'a> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_bytes(self.0);
    }
}
impl<T: Encode> Encode for &[T] {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(self.len(), |enc| {
            for item in *self {
                item.encode(enc);
            }
        });
    }
}
impl<T: Encode, const N: usize> Encode for &[T; N] {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(N, |enc| {
            for item in *self {
                item.encode(enc);
            }
        });
    }
}
impl Encode for &str {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_string(*self);
    }
}
impl Encode for String {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_string(self);
    }
}
impl<T: Encode> Encode for &T {
    fn encode(&self, encoder: &mut impl Encoder) {
        (*self).encode(encoder);
    }
}
impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(self.len(), |enc| {
            for item in self {
                item.encode(enc);
            }
        });
    }
}
impl<T: Encode> Encode for Option<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        match self {
            Some(data) => encoder.write_enum("Option", 1, "Some", |enc| {
                data.encode(enc);
            }),
            None => encoder.write_enum("Option", 0, "None", |_| {}),
        }
    }
}
impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn encode(&self, encoder: &mut impl Encoder) {
        match self {
            Ok(val) => encoder.write_enum("Result", 0, "Ok", |enc| {
                val.encode(enc);
            }),
            Err(err) => encoder.write_enum("Result", 1, "Err", |enc| {
                err.encode(enc);
            })
        }
    }
}
impl Encode for () {
    fn encode(&self, _encoder: &mut impl Encoder) {
        // No need to do anything for binary impl
    }
}
impl<T: Encode> Encode for PhantomData<T> {
    fn encode(&self, _encoder: &mut impl Encoder) {
        // No need to do anything for binary impl
    }
}
impl<T: Encode> Encode for Box<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        (**self).encode(encoder);
    }
}
impl<'a, B> Encode for Cow<'a, B>
where
    B: ToOwned + ?Sized,
    B: Encode
{
    fn encode(&self, encoder: &mut impl Encoder) {
        (**self).encode(encoder);
    }
}
impl<T: Encode> Encode for HashSet<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(self.len(), |enc| {
            for val in self {
                val.encode(enc);
            }
        });
    }
}
impl<K: Encode, V: Encode> Encode for HashMap<K, V> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(self.len(), |enc| {
            for (k, v) in self {
                k.encode(enc);
                v.encode(enc);
            }
        });
    }
}
impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_seq(self.len(), |enc| {
            for (k, v) in self {
                k.encode(enc);
                v.encode(enc);
            }
        });
    }
}
// Add more implementations as use cases come up

trait Encoder {
    fn write_u8(&mut self, n: u8);
    fn write_u16(&mut self, n: u16);
    fn write_u32(&mut self, n: u32);
    fn write_u64(&mut self, n: u64);
    fn write_u128(&mut self, n: u128);
    fn write_i8(&mut self, n: i8);
    fn write_i16(&mut self, n: i16);
    fn write_i32(&mut self, n: i32);
    fn write_i64(&mut self, n: i64);
    fn write_i128(&mut self, n: i128);
    fn write_f32(&mut self, n: f32);
    fn write_f64(&mut self, n: f64);
    fn write_bool(&mut self, b: bool);
    fn write_bytes(&mut self, bytes: &[u8]);
    fn write_string(&mut self, s: &str);
    fn write_seq<F>(&mut self, len: usize, f: F)
    where F: FnOnce(&mut Self);
    fn write_tuple<F>(&mut self, len: usize, f: F)
    where F: FnOnce(&mut Self);
    fn write_struct<F>(&mut self, name: &str, fields: &[&str], f: F)
    where F: FnOnce(&mut Self);
    fn write_enum<F>(&mut self, enum_name: &str, variant_index: u32, variant_name: &str, f: F)
    where F: FnOnce(&mut Self);
}

trait Decode {
    
}

trait Decoder {
    
}