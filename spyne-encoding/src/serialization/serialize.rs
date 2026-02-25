use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, marker::PhantomData};

use crate::serialization::Bytes;

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
    fn write_map<F>(&mut self, len: usize, f: F)
    where F: FnOnce(&mut Self);
    fn write_tuple<F>(&mut self, len: usize, f: F)
    where F: FnOnce(&mut Self);
    fn write_struct<F>(&mut self, name: &str, fields: &[&str], f: F)
    where F: FnOnce(&mut Self);
    fn write_enum<F>(&mut self, enum_name: &str, variant_index: u32, variant_name: &str, f: F)
    where F: FnOnce(&mut Self);
}

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
        serializer.write_map(self.len(), |enc| {
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
        serializer.write_map(self.len(), |enc| {
            for (k, v) in self {
                k.serialize(enc);
                v.serialize(enc);
            }
        });
    }
}
// Add more implementations as use cases come up