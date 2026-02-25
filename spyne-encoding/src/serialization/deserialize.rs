use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, hash::Hash, marker::PhantomData};

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
    fn read_map<F, T>(&mut self, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, usize) -> Result<T, String>;
    fn read_tuple<F, T>(&mut self, len: usize, f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String>;
    fn read_struct<F, T>(&mut self, name: &str, fields: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self) -> Result<T, String>;
    fn read_enum<F, T>(&mut self, enum_name: &str, variants: &[&str], f: F) -> Result<T, String>
    where F: FnOnce(&mut Self, u32) -> Result<T, String>;
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
        deserializer.read_map(|dec, len| {
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
        deserializer.read_map(|dec, len| {
            let mut bm = BTreeMap::new();
            for _ in 0..len {
                bm.insert(K::deserialize(dec)?, V::deserialize(dec)?);
            }
            
            Ok(bm)
        })
    }
}
// Add more implementations as use cases come up, as well as returning borrowed data (optimization)