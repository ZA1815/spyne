mod quote;

use proc_macro::TokenStream;
use crate::quote::{from_stream, quote_help, to_stream};

#[proc_macro]
pub fn quote(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let output = quote_help(input);
    to_stream(output)
}