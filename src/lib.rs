//! This crate provides ways to create dynamic library crates and load them in a [rhai engine](https://docs.rs/rhai/latest/rhai/struct.Engine.html) at runtime.
//! It does it by exposing traits to create plugins and plugin loaders.
//!
//! Check out the "simple" example to get started.
//! Check out the [`plugin`] and [`plugin_loader`] modules to implement your own plugins and loaders.

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
