//! Platform-specific entry points and utilities.

#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
pub(crate) mod android;

#[cfg(not(target_os = "android"))]
pub use desktop::run_desktop;

#[cfg(target_os = "android")]
pub use android::android_main;
