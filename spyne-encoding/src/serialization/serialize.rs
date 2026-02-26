use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, marker::PhantomData};

use crate::serialization::Bytes;

pub trait Serializer<'a> {
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
    fn write_struct<F>(&mut self, name: &'a str, fields: &'a [&'a str], f: F)
    where F: FnOnce(&mut Self);
    fn write_enum<F>(&mut self, enum_name: &'a str, variant_index: u32, variant_name: &'a [&'a str], f: F)
    where F: FnOnce(&mut Self);
}

pub trait Serialize<'a> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>);
}

impl<'a> Serialize<'a> for u8 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u8(*self);
    }
}
impl<'a> Serialize<'a> for u16 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u16(*self);
    }
}
impl<'a> Serialize<'a> for u32 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u32(*self);
    }
}
impl<'a> Serialize<'a> for u64 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u64(*self);
    }
}
impl<'a> Serialize<'a> for u128 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u128(*self);
    }
}
impl<'a> Serialize<'a> for usize {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u64(*self as u64);
    }
}
impl<'a> Serialize<'a> for i8 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i8(*self);
    }
}
impl<'a> Serialize<'a> for i16 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i16(*self);
    }
}
impl<'a> Serialize<'a> for i32 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i32(*self);
    }
}
impl<'a> Serialize<'a> for i64 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i64(*self);
    }
}
impl<'a> Serialize<'a> for i128 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i128(*self);
    }
}
impl<'a> Serialize<'a> for isize {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_i64(*self as i64);
    }
}
impl<'a> Serialize<'a> for f32 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_f32(*self);
    }
}
impl<'a> Serialize<'a> for f64 {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_f64(*self);
    }
}
impl<'a> Serialize<'a> for bool {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_bool(*self);
    }
}
impl<'a> Serialize<'a> for char {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_u32(*self as u32);
    }
}
impl<'a> Serialize<'a> for Bytes<'a> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_bytes(self.0);
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for &[T] {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_seq(self.len(), |enc| {
            for item in *self {
                item.serialize(enc);
            }
        });
    }
}
impl<'a, T: Serialize<'a>, const N: usize> Serialize<'a> for &[T; N] {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_seq(N, |enc| {
            for item in *self {
                item.serialize(enc);
            }
        });
    }
}
impl<'a> Serialize<'a> for &str {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_string(*self);
    }
}
impl<'a> Serialize<'a> for String {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_string(self);
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for &T {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        (*self).serialize(serializer);
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for Vec<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_seq(self.len(), |enc| {
            for item in self {
                item.serialize(enc);
            }
        });
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for Option<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        match self {
            Some(data) => serializer.write_enum("Option", 1, &["Some"], |enc| {
                data.serialize(enc);
            }),
            None => serializer.write_enum("Option", 0, &["None"], |_| {}),
        }
    }
}
impl<'a, T: Serialize<'a>, E: Serialize<'a>> Serialize<'a> for Result<T, E> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        match self {
            Ok(val) => serializer.write_enum("Result", 0, &["Ok"], |enc| {
                val.serialize(enc);
            }),
            Err(err) => serializer.write_enum("Result", 1, &["Err"], |enc| {
                err.serialize(enc);
            })
        }
    }
}
impl<'a> Serialize<'a> for () {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_tuple(0, |_| {});
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for PhantomData<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_tuple(0, |_| {});
        // Treat it like a tuple, if a conflict arises, fix
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for Box<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        (**self).serialize(serializer);
    }
}
impl<'a, B> Serialize<'a> for Cow<'a, B>
where B: ToOwned + ?Sized, B: Serialize<'a> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        (**self).serialize(serializer);
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for HashSet<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_seq(self.len(), |enc| {
            for val in self {
                val.serialize(enc);
            }
        });
    }
}
impl<'a, K: Serialize<'a>, V: Serialize<'a>> Serialize<'a> for HashMap<K, V> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_map(self.len(), |enc| {
            for (k, v) in self {
                k.serialize(enc);
                v.serialize(enc);
            }
        });
    }
}
impl<'a, T: Serialize<'a>> Serialize<'a> for BTreeSet<T> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_seq(self.len(), |enc| {
            for val in self {
                val.serialize(enc);
            }
        });
    }
}
impl<'a, K: Serialize<'a>, V: Serialize<'a>> Serialize<'a> for BTreeMap<K, V> {
    fn serialize(&self, serializer: &mut impl Serializer<'a>) {
        serializer.write_map(self.len(), |enc| {
            for (k, v) in self {
                k.serialize(enc);
                v.serialize(enc);
            }
        });
    }
}
// Add more implementations as use cases come up