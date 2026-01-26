use spyne::serialization::BinarySerde;
use spyne::macros::serialization::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TestStruct {
    a: Vec<u32>,
    b: bool,
    c: i64
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TestEnum {
    Unit,
    Tuple(u32, Vec<i32>),
    Struct { x: u8, y: u16}
}

#[test]
fn test_serde() {
    let test_struct = TestStruct {
        a: vec![5, 3, 10],
        b: false,
        c: 3
    };
    let test_enum_1 = TestEnum::Unit;
    let test_enum_2 = TestEnum::Tuple(3, vec![4, 5, 12]);
    let test_enum_3 = TestEnum::Struct { x: 9, y: 15 };
    
    let ser_struct = match BinarySerde::serialize(&test_struct) {
        Ok(ser) => ser,
        Err(err) => panic!("ERROR SER STRUCT: {:?}", err)
    };
    assert_eq!(ser_struct.clone(), vec![3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
    
    let de_struct = match BinarySerde::deserialize::<TestStruct>(ser_struct) {
        Ok(de) => de,
        Err(err) => panic!("ERROR DE STRUCT: {:?}", err)
    };
    assert_eq!(test_struct, de_struct);
    
    let ser_enum_1 = match BinarySerde::serialize(&test_enum_1) {
        Ok(ser) => ser,
        Err(err) => panic!("ERROR SER ENUM UNIT: {:?}", err)
    };
    assert_eq!(ser_enum_1, vec![0, 0, 0, 0]);
    
    let de_enum_1: TestEnum = match BinarySerde::deserialize::<TestEnum>(ser_enum_1) {
        Ok(de) => de,
        Err(err) => panic!("ERROR DE ENUM UNIT: {:?}", err)
    };
    assert_eq!(test_enum_1, de_enum_1);
    
    let ser_enum_2 = match BinarySerde::serialize(&test_enum_2) {
        Ok(ser) => ser,
        Err(err) => panic!("ERROR SER ENUM TUPLE: {:?}", err)
    };
    assert_eq!(ser_enum_2, vec![1, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 12, 0, 0, 0]);
    
    let de_enum_2: TestEnum = match BinarySerde::deserialize::<TestEnum>(ser_enum_2) {
        Ok(de) => de,
        Err(err) => panic!("ERROR DE ENUM TUPLE: {:?}", err)
    };
    assert_eq!(test_enum_2, de_enum_2);
    
    let ser_enum_3 = match BinarySerde::serialize(&test_enum_3) {
        Ok(ser) => ser,
        Err(err) => panic!("ERROR SER ENUM STRUCT: {:?}", err)
    };
    assert_eq!(ser_enum_3, vec![2, 0, 0, 0, 9, 15, 0]);
    
    let de_enum_3: TestEnum = match BinarySerde::deserialize::<TestEnum>(ser_enum_3) {
        Ok(de) => de,
        Err(err) => panic!("ERROR DE ENUM STRUCT: {:?}", err)
    };
    assert_eq!(test_enum_3, de_enum_3);
}