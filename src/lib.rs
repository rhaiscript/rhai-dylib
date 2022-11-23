#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "windows"))]
compile_error!("unsupported platform - only Linux & Windows are supported");

/// Trait implementation to create objects that load plugins.
pub mod loader;
/// A Rhai module resolver loading dynamic libraries.
pub mod module_resolvers;

/// Re-exporting rhai to prevent version mismatch.
pub use rhai;
