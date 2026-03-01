#[cfg(all(feature = "ffi-c-linux", target_os = "linux"))]
pub mod linux;

#[cfg(all(feature = "ffi-c-macos", target_os = "macos"))]
pub mod macos;