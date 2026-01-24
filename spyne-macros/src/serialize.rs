use spyne_quote::quote;
use spyne_syntax::{ast::{ParsedEnum, ParsedStruct}, token::{Span, TokenIter, TokenTree}};

pub fn serialize_help(data: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut vec: Vec<TokenTree> = Vec::new();
    let mut iter = TokenIter::new(data);
    while let Some(tok) = iter.next() {
        match tok {
            TokenTree::Ident(i, _) if i == "struct" => vec.extend(serialize_struct(&mut iter)),
            TokenTree::Ident(i, _) if i == "enum" => vec.extend(serialize_enum(&mut iter)),
            _ => ()
        }
    }
    
    vec
}

fn serialize_struct(iter: &mut TokenIter) -> Vec<TokenTree> {
    let parsed_struct = ParsedStruct::parse(iter)
        .expect("DeriveSerialize: Struct couldn't be parsed correctly.");
    
    let struct_name_ident = TokenTree::Ident(parsed_struct.name.clone(), Span::default());
    let struct_name_lit = TokenTree::Literal(parsed_struct.name.clone(), Span::default());
    let mut struct_fields_ident: Vec<TokenTree> = Vec::new();
    for field in parsed_struct.fields.iter().clone() {
        match &field.name {
            Some(name) => struct_fields_ident.push(TokenTree::Ident(name.clone(), field.span)),
            None => ()
        }
    }
    let mut struct_fields_lit: Vec<TokenTree> = Vec::new();
    for field in parsed_struct.fields {
        match &field.name {
            Some(name) => struct_fields_lit.push(TokenTree::Literal(name.clone(), field.span)),
            None => ()
        }
    }
    
    quote! {
        impl Serialize for [$struct_name_ident] {
            fn serialize(&self, serializer: &mut impl Serializer) {
                serializer.write_struct([$struct_name_lit], &[ ($ [$struct_fields_lit] ),* ], |ser| {
                    ($ self.[$struct_fields_ident].serialize(ser) );*
                });
            }
        }
    }
}

fn serialize_enum(iter: &mut TokenIter) -> Vec<TokenTree> {
    let parsed_enum = ParsedEnum::parse(iter)
        .expect("DeriveSerialize: Enum couldn't be parsed correctly.");
    
    let enum_name_ident = TokenTree::Ident(parsed_enum.name.clone(), Span::default());
    let enum_name_literal = TokenTree::Literal(parsed_enum.name.clone(), Span::default());
}