//! # Loader.
//!
//! A [`Loader`] is a simple trait that will be used to load a rhai module from a path.

/// A loader using the [`libloading`](https://github.com/nagisa/rust_libloading) crate.
#[cfg(feature = "libloading")]
pub mod libloading;

/// A trait to implement an object that loads Rhai modules.
pub trait Loader {
    /// Load a module from a path and apply it to a [`rhai::Engine`].
    #[allow(clippy::missing_errors_doc)]
    fn load(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>>;
}
