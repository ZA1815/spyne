use spyne_quote::quote;
use spyne_syntax::{ast::{ParsedEnum, ParsedStruct, VariantData}, token::{Delimiter, Spacing, Span, TokenIter, TokenTree}};

pub fn deserialize_help(data: Vec<TokenTree>) -> Vec<TokenTree> {
   let mut vec: Vec<TokenTree> = Vec::new();
   let mut iter = TokenIter::new(data);
   while let Some(tok) = iter.next() {
       match tok {
           TokenTree::Ident(i, _) if i == "struct" => vec.extend(deserialize_struct(&mut iter)),
           TokenTree::Ident(i, _) if i == "enum" => vec.extend(deserialize_enum(&mut iter)),
           _ => ()
       }
   }
   
   vec
}

fn deserialize_struct(iter: &mut TokenIter) -> Vec<TokenTree> {
    let parsed_struct = ParsedStruct::parse(iter)
        .expect("DeriveDeserialize: Struct couldn't be parsed correctly.");
    let struct_name_ident = TokenTree::Ident(parsed_struct.name.clone(), parsed_struct.span);
    let struct_name_lit = TokenTree::Literal(parsed_struct.name.clone(), parsed_struct.span);
    let mut struct_fields_ident: Vec<TokenTree> = Vec::new();
    let mut struct_fields_lit: Vec<TokenTree> = Vec::new();
    let mut struct_types: Vec<TokenTree> = Vec::new();
    for field in parsed_struct.fields {
        match &field.name {
            Some(name) => {
                struct_fields_ident.push(TokenTree::Ident(name.clone(), field.span));
                struct_fields_lit.push(TokenTree::Literal(name.clone(), field.span));
            }
            None => ()
        }
        struct_types.push(TokenTree::Group(Delimiter::None, field.ty, field.span));
    }
    
    quote! {
        impl Deserialize for [$ struct_name_ident ] {
            fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
                deserializer.read_struct([$ struct_name_lit ], &[($ [$ struct_fields_lit ] ),*], |de| {
                    Ok(Self {
                        ($ [$ struct_fields_ident ]: [$ struct_types]::deserialize(de)? ),*
                    })
                });
            }
        }
    }
}

fn deserialize_enum(iter: &mut TokenIter) -> Vec<TokenTree> {
    let parsed_enum = ParsedEnum::parse(iter)
        .expect("DeriveDeserialize: Enum couldn't be parsed correctly.");
    let enum_name_ident = TokenTree::Ident(parsed_enum.name.clone(), parsed_enum.span);
    let enum_name_lit = TokenTree::Literal(parsed_enum.name.clone(), parsed_enum.span);
    let mut variants: Vec<TokenTree> = Vec::new();
    let mut enum_arms: Vec<TokenTree> = Vec::new();
    for variant in parsed_enum.variants {
        let var_idx = variant.index;
        match variant.data {
            VariantData::Unit(s) => {
                let var_name_ident = TokenTree::Ident(variant.name.clone(), s);
                let var_name_lit = TokenTree::Literal(variant.name.clone(), s);
                variants.push(var_name_lit);
                enum_arms.extend(quote! {
                    [$ var_idx ] => Ok([$ enum_name_ident ]::[$ var_name_ident ])
                });
            }
            VariantData::Tuple(data, s) => {
                let var_name = TokenTree::Ident(variant.name.clone(), s);
                let mut field_types: Vec<TokenTree> = Vec::new();
                let field_num = TokenTree::Literal(format!("{}", data.len()), s);
                for field in data {
                    field_types.push(TokenTree::Group(Delimiter::None, field.ty, field.span));
                }
                
                enum_arms.extend(quote! {
                    [$ var_idx ] => de.read_tuple([$ field_num ], |de| {
                       Ok([$ enum_name_ident ]::[$ var_name ](($ [$ field_types]::deserialize(de)? ),*)) 
                    });
                });
            }
            VariantData::Struct(data, s) => {
                let var_name_ident = TokenTree::Ident(variant.name.clone(), s);
                let var_name_lit = TokenTree::Literal(variant.name.clone(), s);
                let mut field_names_ident: Vec<TokenTree> = Vec::new();
                let mut field_names_lit: Vec<TokenTree> = Vec::new();
                let mut field_types: Vec<TokenTree> = Vec::new();
                for field in data {
                    match &field.name {
                        Some(name) => {
                            field_names_ident.push(TokenTree::Ident(name.clone(), field.span));
                            field_names_lit.push(TokenTree::Literal(name.clone(), field.span));
                        }
                        None => ()
                    }
                    
                    field_types.push(TokenTree::Group(Delimiter::None, field.ty, field.span));
                }
                
                enum_arms.extend(quote! {
                    [$ var_idx ] => de.read_struct([$ var_name_lit ], &[($ [$ field_names_lit ]),*], |de| {
                        Ok([$ enum_name_ident ]::[$ var_name_ident ] { ($ [$ field_names_ident ]: [$ field_types ]::deserialize(de)? ),* } )
                    });
                });
            }
        }
        enum_arms.push(TokenTree::Punct(',', Spacing::Alone, Span::default()));
    }
    
    enum_arms.extend(quote! { _ => Err("DeriveDeserialize: Variant index out of bounds.".to_string()) });
    
    quote! {
        impl Deserialize for [$ enum_name_ident ] {
            fn deserialize(deserializer: &mut impl Deserializer) -> Result<Self, String> {
                deserializer.read_enum([$ enum_name_lit ], &[($ [$ variants ] ),*], |de, idx| {
                    match idx {
                        [$ enum_arms ]
                    }
                });
            }
        }
    }
}