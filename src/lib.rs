//! # rhai-dylib - Load native Rust plugins using dynamic libraries !
//!
//! This crate is a really simple implementation of a dynamic library loader for Rhai.
//! It provides traits to create plugins and plugin loaders.
//!
//! Check out the "simple" example to get started.

#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

/// A plugin trait implementation.
pub mod plugin;
/// Trait implementation to create objects that load plugins.
pub mod plugin_loader;

/// Re-exporting rhai to prevent version mismatch.
pub use rhai;
