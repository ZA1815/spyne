#[cfg(feature = "macros-quote")]
pub use spyne_quote::quote;

#[cfg(feature = "macros-serialization")]
pub use spyne_macros::{Serialize, Deserialize};