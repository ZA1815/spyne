use spyne_quote::quote;
use spyne_syntax::{
    ast::{ParsedAttribute, ParsedStruct},
    token::{Delimiter, TokenIter, TokenTree},
};

pub fn vulkan_functions_help(data: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut vec: Vec<TokenTree> = Vec::new();
    let mut iter = TokenIter::new(data);
    let mut attrs: Option<ParsedAttribute> = None;
    while let Some(tok) = iter.next() {
        match tok {
            TokenTree::Ident(s, _) if s == "struct" => {
                vec.extend(vulkan_struct_help(&mut iter, attrs.take()))
            }
            TokenTree::Punct(c, _, _) if *c == '#' => {
                attrs = Some(ParsedAttribute::parse(&mut iter).expect("VulkanFunctions: Struct attributes couldn't be parsed correctly."));
            }
            _ => (),
        }
    }

    vec
}

fn vulkan_struct_help(
    iter: &mut TokenIter,
    header_attrs: Option<ParsedAttribute>,
) -> Vec<TokenTree> {
    let parsed_struct =
        ParsedStruct::parse(iter).expect("VulkanFunctions: Struct couldn't be parsed correctly.");

    let struct_name = TokenTree::Ident(parsed_struct.name, parsed_struct.span);
    let mut field_names: Vec<TokenTree> = Vec::new();
    let mut field_attrs: Vec<TokenTree> = Vec::new();
    let mut field_types: Vec<TokenTree> = Vec::new();
    for field in parsed_struct.fields {
        field_names.push(TokenTree::Ident(field.name.unwrap(), field.span));
        for attr in field.attrs {
            if attr.name == "cfg" {
                continue;
            }
            field_attrs.push(
                attr.args
                    .get("name")
                    .expect("VulkanFunctions: Attribute key 'name' wasn't found on field.")
                    .to_owned()
                    .first()
                    .expect("VulkanFunctions: Value for attribute key 'name' wasn't found on field.")
                    .to_owned()
            )
        }
        field_types.push(TokenTree::Group(Delimiter::None, field.ty, field.span));
    }
    
    let (handle, loader) = match header_attrs {
        Some(attr) => (
            attr.args
                .get("handle")
                .expect("VulkanFunctions: Attribute key 'handle' wasn't found.")
                .to_owned()
                .first()
                .expect("VulkanFunctions: Value for attribute key 'handle' wasn't found.")
                .to_owned(),
            attr.args
                .get("loader")
                .expect("VulkanFunctions: Attribute key 'loader' wasn't found.")
                .to_owned()
                .first()
                .expect("VulkanFunctions: Value for attribute key 'loader' wasn't found.")
                .to_owned()
        ),
        None => panic!("VulkanFunctions: Struct attributes weren't found."),
    };
    
    quote! {
        impl [$ struct_name ] {
            pub unsafe fn load(loader: [$ loader ], handle: [$ handle ]) -> Self {
                ($ let [$ field_names.clone() ]: [$ field_types ] = unsafe { transmute(loader(handle, CString::new([$ field_attrs.to_owned() ]).unwrap().as_ptr())) }; )*
                
                Self {
                    ($ [$ field_names ] ),*
                }
            }
        }
    }
}