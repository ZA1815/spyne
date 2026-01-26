use spyne::primitives::serialization::BinarySerde;
use spyne::macros::serialization::{Deserialize, Serialize};

// Add pub to struct to test later
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TestStruct {
    a: Vec<u32>,
    b: bool,
    c: i64
}