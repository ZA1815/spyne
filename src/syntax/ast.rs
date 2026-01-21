use crate::syntax::token::TokenTree;

pub struct ParsedStruct {
    pub name: String,
    pub fields: Vec<ParsedField>
}

pub struct ParsedEnum {
    pub name: String,
    pub variants: Vec<ParsedVariant>
}

pub struct ParsedField {
    pub name: Option<String>,
    pub ty: Vec<TokenTree>
}

pub struct ParsedVariant {
    pub name: String,
    pub index: u32,
    pub data: VariantData
}

pub enum VariantData {
    Unit,
    Tuple(Vec<ParsedField>),
    Struct(Vec<ParsedField>)
}