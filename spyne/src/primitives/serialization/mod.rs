// #[cfg(feature = "serialization-binary")]
mod binary;
// #[cfg(feature = "serialization-binary")]
pub use binary::BinarySerde;

use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, hash::Hash, marker::PhantomData};

pub struct Bytes<'a>(pub &'a [u8]);

pub trait Serialize {
    fn serialize(&self, serializer: &mut impl Serializer);
}

impl Serialize for u8 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u8(*self);
    }
}
impl Serialize for u16 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u16(*self);
    }
}
impl Serialize for u32 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u32(*self);
    }
}
impl Serialize for u64 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u64(*self);
    }
}
impl Serialize for u128 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u128(*self);
    }
}
impl Serialize for usize {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u64(*self as u64);
    }
}
impl Serialize for i8 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i8(*self);
    }
}
impl Serialize for i16 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i16(*self);
    }
}
impl Serialize for i32 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i32(*self);
    }
}
impl Serialize for i64 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i64(*self);
    }
}
impl Serialize for i128 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i128(*self);
    }
}
impl Serialize for isize {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_i64(*self as i64);
    }
}
impl Serialize for f32 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_f32(*self);
    }
}
impl Serialize for f64 {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_f64(*self);
    }
}
impl Serialize for bool {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_bool(*self);
    }
}
impl Serialize for char {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_u32(*self as u32);
    }
}
impl<'a> Serialize for Bytes<'a> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_bytes(self.0);
    }
}
impl<T: Serialize> Serialize for &[T] {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for item in *self {
                item.serialize(enc);
            }
        });
    }
}
impl<T: Serialize, const N: usize> Serialize for &[T; N] {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(N, |enc| {
            for item in *self {
                item.serialize(enc);
            }
        });
    }
}
impl Serialize for &str {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_string(*self);
    }
}
impl Serialize for String {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_string(self);
    }
}
impl<T: Serialize> Serialize for &T {
    fn serialize(&self, serializer: &mut impl Serializer) {
        (*self).serialize(serializer);
    }
}
impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for item in self {
                item.serialize(enc);
            }
        });
    }
}
impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        match self {
            Some(data) => serializer.write_enum("Option", 1, "Some", |enc| {
                data.serialize(enc);
            }),
            None => serializer.write_enum("Option", 0, "None", |_| {}),
        }
    }
}
impl<T: Serialize, E: Serialize> Serialize for Result<T, E> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        match self {
            Ok(val) => serializer.write_enum("Result", 0, "Ok", |enc| {
                val.serialize(enc);
            }),
            Err(err) => serializer.write_enum("Result", 1, "Err", |enc| {
                err.serialize(enc);
            })
        }
    }
}
impl Serialize for () {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_tuple(0, |_| {});
    }
}
impl<T: Serialize> Serialize for PhantomData<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_tuple(0, |_| {});
        // Treat it like a tuple, if a conflict arises, fix
    }
}
impl<T: Serialize> Serialize for Box<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        (**self).serialize(serializer);
    }
}
impl<'a, B> Serialize for Cow<'a, B>
where B: ToOwned + ?Sized, B: Serialize {
    fn serialize(&self, serializer: &mut impl Serializer) {
        (**self).serialize(serializer);
    }
}
impl<T: Serialize> Serialize for HashSet<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for val in self {
                val.serialize(enc);
            }
        });
    }
}
impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for (k, v) in self {
                k.serialize(enc);
                v.serialize(enc);
            }
        });
    }
}
impl<T: Serialize> Serialize for BTreeSet<T> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for val in self {
                val.serialize(enc);
            }
        });
    }
}
impl<K: Serialize, V: Serialize> Serialize for BTreeMap<K, V> {
    fn serialize(&self, serializer: &mut impl Serializer) {
        serializer.write_seq(self.len(), |enc| {
            for (k, v) in self {
                k.serialize(enc);
                v.serialize(enc);
            }
        });
    }
}
// Add more implementations as use cases come up

pub trait Serializer {
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

pub trait Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String>
    where Self: Sized;
}

impl Deserialize for u8 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u8()
    }
}
impl Deserialize for u16 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u16()
    }
}
impl Deserialize for u32 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u32()
    }
}
impl Deserialize for u64 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u64()
    }
}
impl Deserialize for u128 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u128()
    }
}
impl Deserialize for usize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u64().map(|n| n as usize)
    }
}
impl Deserialize for i8 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i8()
    }
}
impl Deserialize for i16 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i16()
    }
}
impl Deserialize for i32 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i32()
    }
}
impl Deserialize for i64 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i64()
    }
}
impl Deserialize for i128 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i128()
    }
}
impl Deserialize for isize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_i64().map(|n| n as isize)
    }
}
impl Deserialize for f32 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_f32()
    }
}
impl Deserialize for f64 {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_f64()
    }
}
impl Deserialize for bool {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_bool()
    }
}
impl Deserialize for char {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_u32().and_then(|n| char::from_u32(n).ok_or_else(||format!("BinarySerde: Error during deserialization (char not a valid Unicode scalar: {})", n))) 
    }
}
impl Deserialize for String {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_string()
    }
}
impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_seq(|dec, len| {
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::deserialize(dec)?);
            }
            
            Ok(vec)
        })
    }
}
impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_enum("Option", &["None", "Some"], |dec, var| {
            match var {
                0 => Ok(None),
                1 => Ok(Some(T::deserialize(dec)?)),
                _ => Err(format!("BinarySerde: Variant index out of bounds."))
            }
        })
    }
}
impl<T: Deserialize, E: Deserialize> Deserialize for Result<T, E> {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_enum("Result", &["Ok", "Err"], |dec, var| {
            match var {
                0 => Ok(Ok(T::deserialize(dec)?)),
                1 => Ok(Err(E::deserialize(dec)?)),
                _ => Err(format!("BinarySerde: Variant index out of bounds."))
            }
        })
    }
}
impl Deserialize for () {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_tuple(0, |_| { Ok(()) })
    }
}
impl<T: Deserialize> Deserialize for PhantomData<T> {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_tuple(0, |_| { Ok(PhantomData) })
    }
}
impl<T: Deserialize> Deserialize for Box<T> {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        Ok(Box::new(T::deserialize(deserializer)?))
    }
}
// Change this when borrowed data decoding is implemented
impl<'a, B> Deserialize for Cow<'a, B>
where B: ToOwned + ?Sized, <B as ToOwned>::Owned: Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        Ok(Cow::Owned(<B as ToOwned>::Owned::deserialize(deserializer)?))
    }
}
impl<T> Deserialize for HashSet<T>
where T: Hash + Eq, T: Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_seq(|dec, len| {
            let mut hs = HashSet::with_capacity(len);
            for _ in 0..len {
                hs.insert(T::deserialize(dec)?);
            }
            
            Ok(hs)
        })
    }
}
impl<K, V> Deserialize for HashMap<K, V>
where K: Hash + Eq, K: Deserialize, V: Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_seq(|dec, len| {
            let mut hm = HashMap::with_capacity(len);
            for _ in 0..len {
                hm.insert(K::deserialize(dec)?, V::deserialize(dec)?);
            }
            
            Ok(hm)
        })
    }
}
impl<T> Deserialize for BTreeSet<T>
where T: Ord, T: Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_seq(|dec, len| {
            let mut bs = BTreeSet::new();
            for _ in 0..len {
                bs.insert(T::deserialize(dec)?);
            }
            
            Ok(bs)
        })
    }
}
impl<K, V> Deserialize for BTreeMap<K, V>
where K: Ord, K: Deserialize, V: Deserialize {
    fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
        deserializer.read_seq(|dec, len| {
            let mut bm = BTreeMap::new();
            for _ in 0..len {
                bm.insert(K::deserialize(dec)?, V::deserialize(dec)?);
            }
            
            Ok(bm)
        })
    }
}
// Add more implementations as use cases come up, as well as returning borrowed data (optimization)

pub trait Deserializer {
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