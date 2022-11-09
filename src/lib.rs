//! This crate provides ways to create dynamic library crates and load them in a [rhai engine](https://docs.rs/rhai/latest/rhai/struct.Engine.html) at runtime.
//! It does it by exposing traits to create plugins and plugin loaders.
//!
//! Check out the "simple" example to get started.
//! Check out the [`plugin`] and [`plugin_loader`] modules to implement your own plugins and loaders.
//!
//! # Limitations
//!
//! There are multiple limitations that you need to take into account before using this crate.
//!
//! > TL;DR
//! > To use this crate, you need to:
//! > - Compile **EVERYTHING**, plugins and program that will load them, inside the **SAME** workspace or **WITHOUT** a workspace.
//! > - Use the `rhai::config::hashing::set_ahash_seed` function with the **SAME** four u64 array (i.e. `rhai::config::hashing::set_ahash_seed(Some([1, 2, 3, 4]))`) when building your plugins and the program that will load them.
//!
//! ## Rust ABI
//!
//! This plugin implementation uses Rust's ABI, which is unstable and will change between compiler versions.
//!
//! > A C repr has not yet been provided but is being discussed, probably locked behind a feature gate.
//!
//! This means that all of the plugins that you will use in your main program need to be compiled with the **EXACT** same
//! compiler version.
//!
//! ## TypeId
//!
//! Rust [`TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html) is an object used to compare types at compile time. Rhai uses those to check which type a [`Dynamic`](https://docs.rs/rhai/1.10.1/rhai/struct.Dynamic.html) object is. This is a problem for dynamic libraries because `TypeIds` sometime change between compilations.
//!
//! That means that in certain situations, Rhai cannot compare two types, even tough they are the same, because the `TypeId` of said types is different between the plugin and the binary.
//!
//! To fix this, you will need to compile your main binary **AND** plugins inside the **SAME** workspace, or compile everything **OUTSIDE** of a workspace. Compiling, for example, a binary in a workspace, and a plugin outside will probably result in `TypeIds` mismatch.
//!
//! >  You can use
//! > ```rust
//! > println!("{:?}", std::any::TypeId::of::<rhai::Map>());
//! > ```
//! > In your binary & plugins to check the type id value.
//!
//! If you have any idea of how the compiler generates those typeids between workspaces and single crates, please help us complete this readme !
//!
//! ## Hashing
//!
//! Rhai uses the [`ahash`](https://github.com/tkaitchuck/ahash) crate under the hood to create identifiers for function calls. For each compilation of your code, a new seed is generated when hashing the types. Therefore, compiling your main program and your plugin different times will result in a hash mismatch, meaning that you won't be able to call the API of your plugin.
//!
//! To bypass that, you need to use the `rhai::config::hashing::set_ahash_seed` function with an array of four `u64`.
//!
//! Beware that the `rhai::config::hashing::set_ahash_seed` function is only available from the main branch of Rhai for the time being.

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
