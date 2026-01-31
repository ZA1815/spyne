pub mod linux;

pub mod macos;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("Windows is not supported with C FFI");