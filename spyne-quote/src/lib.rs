mod quote;

use crate::quote::{from_stream, quote_help, to_stream};
use proc_macro::TokenStream;
use spyne_syntax::token::{Spacing, Span, TokenTree};

#[proc_macro]
pub fn quote(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let root: Vec<TokenTree> = vec![
        TokenTree::Ident(format!("spyne"), Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Punct(':', Spacing::Joint, Span::default()),
        TokenTree::Ident(format!("syntax"), Span::default())
    ];
    let output = quote_help(input, root);
    to_stream(output)
}

#[proc_macro]
pub fn quote_internal(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let root: Vec<TokenTree> = vec![
        TokenTree::Ident(format!("spyne_syntax"), Span::default()),
    ];
    let output = quote_help(input, root);
    to_stream(output)
}