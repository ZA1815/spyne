use spyne_quote::quote;
use spyne_syntax::{ast::{ParsedEnum, ParsedStruct, VariantData}, token::{Spacing, Span, TokenIter, TokenTree}};

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
    
    let struct_name_ident = TokenTree::Ident(parsed_struct.name.clone(), parsed_struct.span);
    let struct_name_lit = TokenTree::Literal(parsed_struct.name.clone(), parsed_struct.span);
    let mut struct_fields_ident: Vec<TokenTree> = Vec::new();
    let mut struct_fields_lit: Vec<TokenTree> = Vec::new();
    for field in parsed_struct.fields {
        match &field.name {
            Some(name) => {
                struct_fields_ident.push(TokenTree::Ident(name.clone(), field.span));
                struct_fields_lit.push(TokenTree::Literal(name.clone(), field.span));
            }
            None => ()
        }
    }
    
    quote! {
        impl Serialize for [$ struct_name_ident ] {
            fn serialize(&self, serializer: &mut impl Serializer) {
                serializer.write_struct([$ struct_name_lit ], &[($ [$ struct_fields_lit ] ),*], |ser| {
                    ($ self.[$ struct_fields_ident ].serialize(ser) );*
                });
            }
        }
    }
}

fn serialize_enum(iter: &mut TokenIter) -> Vec<TokenTree> {
    let parsed_enum = ParsedEnum::parse(iter)
        .expect("DeriveSerialize: Enum couldn't be parsed correctly.");
    
    let enum_name_ident = TokenTree::Ident(parsed_enum.name.clone(), parsed_enum.span);
    let enum_name_lit = TokenTree::Literal(parsed_enum.name.clone(), parsed_enum.span);
    let mut enum_arms: Vec<TokenTree> = Vec::new();
    for variant in parsed_enum.variants {
        let var_idx = TokenTree::Literal(format!("{}", variant.index), Span::default());
        match variant.data {
            VariantData::Unit(s) => {
                let var_name_ident = TokenTree::Ident(variant.name.clone(), s);
                let var_name_lit = TokenTree::Literal(variant.name.clone(), s);
               enum_arms.extend(quote! {
                   Self::[$ var_name_ident ] => serializer.write_enum([$ enum_name_lit ], [$ var_idx ], [$ var_name_lit ], |_| {})
               }); 
            }
            VariantData::Tuple(data, s) => {
                let var_name_ident = TokenTree::Ident(variant.name.clone(), s);
                let var_name_lit = TokenTree::Literal(variant.name.clone(), s);
                let mut field_names: Vec<TokenTree> = Vec::new();
                let field_num = TokenTree::Literal(format!("{}", data.len()), Span::default());
                for i in 0..data.len() {
                    field_names.push(TokenTree::Ident(format!("f{}", i), Span::default()));
                }
                enum_arms.extend(quote! {
                    Self::[$ var_name_ident ](($ [$ field_names.clone() ] ),*) => serializer.write_enum([$ enum_name_lit ], [$ var_idx ], [$ var_name_lit ], |ser| {
                        ser.write_tuple([$ field_num ], |ser| {
                            ($ [$ field_names ].serialize(ser) );*
                        });
                    }),
                });
            }
            VariantData::Struct(data, s) => {
                let var_name_ident = TokenTree::Ident(variant.name.clone(), s);
                let var_name_lit = TokenTree::Literal(variant.name.clone(), s);
                let mut field_names_ident: Vec<TokenTree> = Vec::new();
                let mut field_names_lit: Vec<TokenTree> = Vec::new();
                for field in data {
                    match &field.name {
                        Some(name) => {
                            field_names_ident.push(TokenTree::Ident(name.clone(), field.span));
                            field_names_lit.push(TokenTree::Literal(name.clone(), field.span));
                        }
                        None => ()
                    }
                }
                enum_arms.extend(quote! {
                    Self::[$ var_name_ident ] { ($ [$ field_names_ident.clone() ] ),* } => serializer.write_enum([$ enum_name_lit ], [$ var_idx ], [$ var_name_lit ], |ser| {
                        ser.write_struct([$ var_name_lit ], &[($ [$ field_names_lit ] ),*], |ser| {
                            ($ [$ field_names_ident ].serialize(ser) );*
                        });
                    });
                });
            }
        }
        
        enum_arms.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
    }
    
    quote! {
        impl Serialize for [ $enum_name_ident ] {
            fn serialize(&self, serializer: &mut impl Serializer) {
                match self {
                    [$ enum_arms ]
                }
            }
        }
    }
}