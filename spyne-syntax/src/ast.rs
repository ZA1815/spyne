use crate::token::{Span, TokenTree};

pub struct ParsedStruct {
    pub name: String,
    pub fields: Vec<ParsedField>,
    pub span: Span
}

pub struct ParsedEnum {
    pub name: String,
    pub variants: Vec<ParsedVariant>,
    pub span: Span
}

pub struct ParsedField {
    pub name: Option<String>,
    pub ty: Vec<TokenTree>,
    pub span: Span
}

pub struct ParsedVariant {
    pub name: String,
    pub index: u32,
    pub data: VariantData,
    pub span: Span
}

pub enum VariantData {
    Unit(Span),
    Tuple(Vec<ParsedField>, Span),
    Struct(Vec<ParsedField>, Span)
}