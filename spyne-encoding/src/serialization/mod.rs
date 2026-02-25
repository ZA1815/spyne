pub mod binary;
pub mod deserialize;
pub mod json;
pub mod serialize;

pub struct Bytes<'a>(pub &'a [u8]);



