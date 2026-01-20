// #[cfg(feature = "serialization-binary")]
mod binary;
// #[cfg(feature = "serialization-binary")]
pub use binary::BinarySerde;

use std::{borrow::Cow, collections::{BTreeMap, HashMap, HashSet}, hash::Hash, marker::PhantomData};

pub struct Bytes<'a>(pub &'a [u8]);

pub trait Encode {
    fn encode(&self, encoder: &mut impl Encoder);
}

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
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_tuple(0, |_| {});
    }
}
impl<T: Encode> Encode for PhantomData<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        encoder.write_tuple(0, |_| {});
        // Treat it like a tuple, if a conflict arises, fix
    }
}
impl<T: Encode> Encode for Box<T> {
    fn encode(&self, encoder: &mut impl Encoder) {
        (**self).encode(encoder);
    }
}
impl<'a, B> Encode for Cow<'a, B>
where B: ToOwned + ?Sized, B: Encode {
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

pub trait Encoder {
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

pub trait Decode {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String>
    where Self: Sized;
}

impl Decode for u8 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u8()
    }
}
impl Decode for u16 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u16()
    }
}
impl Decode for u32 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u32()
    }
}
impl Decode for u64 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u64()
    }
}
impl Decode for u128 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u128()
    }
}
impl Decode for usize {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u64().map(|n| n as usize)
    }
}
impl Decode for i8 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i8()
    }
}
impl Decode for i16 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i16()
    }
}
impl Decode for i32 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i32()
    }
}
impl Decode for i64 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i64()
    }
}
impl Decode for i128 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i128()
    }
}
impl Decode for isize {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_i64().map(|n| n as isize)
    }
}
impl Decode for f32 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_f32()
    }
}
impl Decode for f64 {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_f64()
    }
}
impl Decode for bool {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_bool()
    }
}
impl Decode for char {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_u32().and_then(|n| char::from_u32(n).ok_or_else(||format!("BinarySerde: Error during deserialization (char not a valid Unicode scalar: {})", n))) 
    }
}
impl Decode for String {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_string()
    }
}
impl<T: Decode> Decode for Vec<T> {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_seq(|dec, len| {
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode(dec)?);
            }
            
            Ok(vec)
        })
    }
}
impl<T: Decode> Decode for Option<T> {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_enum("Option", &["None", "Some"], |dec, var| {
            match var {
                0 => Ok(None),
                1 => Ok(Some(T::decode(dec)?)),
                _ => Err(format!("BinarySerde: Variant index out of bounds."))
            }
        })
    }
}
impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_enum("Result", &["Ok", "Err"], |dec, var| {
            match var {
                0 => Ok(Ok(T::decode(dec)?)),
                1 => Ok(Err(E::decode(dec)?)),
                _ => Err(format!("BinarySerde: Variant index out of bounds."))
            }
        })
    }
}
impl Decode for () {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_tuple(0, |_| { Ok(()) })
    }
}
impl<T: Decode> Decode for PhantomData<T> {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_tuple(0, |_| { Ok(PhantomData) })
    }
}
impl<T: Decode> Decode for Box<T> {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        Ok(Box::new(T::decode(decoder)?))
    }
}
// Change this when borrowed data decoding is implemented
impl<'a, B> Decode for Cow<'a, B>
where B: ToOwned + ?Sized, <B as ToOwned>::Owned: Decode {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        Ok(Cow::Owned(<B as ToOwned>::Owned::decode(decoder)?))
    }
}
impl<T> Decode for HashSet<T>
where T: Hash + Eq, T: Decode {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_seq(|dec, len| {
            let mut hs = HashSet::with_capacity(len);
            for _ in 0..len {
                hs.insert(T::decode(dec)?);
            }
            
            Ok(hs)
        })
    }
}
impl<K, V> Decode for HashMap<K, V>
where K: Hash + Eq, K: Decode, V: Decode {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_seq(|dec, len| {
            let mut hm = HashMap::with_capacity(len);
            for _ in 0..len {
                hm.insert(K::decode(dec)?, V::decode(dec)?);
            }
            
            Ok(hm)
        })
    }
}
impl<K, V> Decode for BTreeMap<K, V>
where K: Ord, K: Decode, V: Decode {
    fn decode(decoder: &mut impl Decoder) -> Result<Self, String> {
        decoder.read_seq(|dec, len| {
            let mut bm = BTreeMap::new();
            for _ in 0..len {
                bm.insert(K::decode(dec)?, V::decode(dec)?);
            }
            
            Ok(bm)
        })
    }
}
// Add more implementations as use cases come up, as well as returning borrowed data (optimization)

pub trait Decoder {
    fn read_u8(&mut self) -> Result<u8, String>;
    fn read_u16(&mut self) -> Result<u16, String>;
    fn read_u32(&mut self) -> Result<u32, String>;
    fn read_u64(&mut self) -> Result<u64, String>;
    fn read_u128(&mut self) -> Result<u128, String>;
    fn read_i8(&mut self) -> Result<i8, String>;
    fn read_i16(&mut self) -> Result<i16, String>;
    fn read_i32(&mut self) -> Result<i32, String>;
    fn read_i64(&mut self) -> Result<i64, String>;
    fn read_i128(&mut self) -> Result<i128, String>;
    fn read_f32(&mut self) -> Result<f32, String>;
    fn read_f64(&mut self) -> Result<f64, String>;
    fn read_bool(&mut self) -> Result<bool, String>;
    fn read_bytes(&mut self) -> Result<Vec<u8>, String>;
    fn read_string(&mut self) -> Result<String, String>;
    fn read_seq<F, T>(&mut self, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, usize) -> Result<T, String>;
    fn read_tuple<F, T>(&mut self, len: usize, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String>;
    fn read_struct<F, T>(&mut self, name: &str, fields: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String>;
    fn read_enum<F, T>(&mut self, enum_name: &str, variants: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, u32) -> Result<T, String>;
}