mod shared;
mod serialize;
mod deserialize;
mod vulkan_functions;

use proc_macro::TokenStream;

use crate::shared::{from_stream, to_stream};
use crate::serialize::serialize_help;
use crate::deserialize::deserialize_help;
use crate::vulkan_functions::vulkan_functions_help;

#[proc_macro_derive(Serialize)]
pub fn serialize(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let output = serialize_help(input);
    to_stream(output)
}

#[proc_macro_derive(Deserialize)]
pub fn deserialize(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let output = deserialize_help(input);
    to_stream(output)
}

#[proc_macro_derive(VulkanFunctions, attributes(vulkan))]
pub fn vulkan_functions(stream: TokenStream) -> TokenStream {
    let input = from_stream(stream);
    let output = vulkan_functions_help(input);
    to_stream(output)
}