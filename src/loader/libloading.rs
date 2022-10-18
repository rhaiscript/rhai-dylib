//! # dylib loader.
//!
//! The [`Libloading`] loader enables you to expend functionality of a [`rhai::Engine`] via dynamic libraries using [`libloading`](https://github.com/nagisa/rust_libloading).
//!
//! You need to declare the entrypoint function of your module, following the [`Entrypoint`] prototype.
//! The name of the function must be the same as [`MODULE_ENTRYPOINT`].
//!
//! ```rust
//! fn module_entrypoint() -> rhai::Shared<rhai::Module> {
//!     // ...
//! }
//! ```
//!
//! You can easily, for example, implement and export your module using Rhai's [plugin modules](https://rhai.rs/book/plugins/module.html).
//!
//! ```rust
//! // Use the `export_module` macro to generate your api.
//! #[rhai::export_module]
//! mod my_api {
//!     pub fn get_num() -> i64 {
//!         3
//!     }
//!     pub fn print_stuff() {
//!        println!("Hello World!");
//!     }
//! }
//!
//! // The entrypoint function of your module.
//! // `extern "C"` can be omitted if you are using the `rust` feature.
//! #[no_mangle]
//! extern "C" fn module_entrypoint() -> rhai::Shared<rhai::Module> {
//!     // Build your module.
//!     rhai::exported_module!(my_api).into()
//! }
//! ```

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "windows"))]
compile_error!("unsupported platform - only Linux & Windows are supported");

use super::{Loader, LoaderError};

/// Entrypoint prototype for a rhai extension.
pub type Entrypoint = fn() -> rhai::Shared<rhai::Module>;
/// The name of the function that will be called to update the [`rhai::Engine`].
pub const MODULE_ENTRYPOINT: &str = "module_entrypoint";

/// Loading dynamic libraries using the [`libloading`](https://github.com/nagisa/rust_libloading) crate.
///
/// # Example
///
/// ```rust
/// // Create your dynamic library loader & rhai engine.
/// let mut loader = Libloading::new();
/// let mut engine = rhai::Engine::new();
///
/// // `my_first_extension` library exposes the `print_first` function.
/// loader.load("my_first_extension.so", &mut engine).expect("failed to load library 1");
/// // `my_second_extension` library exposes the `print_second` function.
/// loader.load("my_second_extension.so", &mut engine).expect("failed to load library 2");
///
/// // functions are now registered in the engine and can be called !
/// engine.run(r"
///     print_first();
///     print_second();
/// ");
/// ```
pub struct Libloading {
    /// Libraries loaded in memory.
    libraries: Vec<libloading::Library>,
}

impl Default for Libloading {
    /// Create a new instance of the loader.
    fn default() -> Self {
        Self { libraries: vec![] }
    }
}

impl Libloading {
    /// Create a new instance of the loader.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Loader for Libloading {
    /// Load a rhai extension from a dynamic library.
    fn load<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<rhai::Shared<rhai::Module>, LoaderError> {
        let library = unsafe {
            #[cfg(target_os = "linux")]
            {
                // Workaround for a crash on library unloading on linux: https://github.com/nagisa/rust_libloading/issues/5#issuecomment-244195096
                libloading::os::unix::Library::open(
                    Some(path.as_ref()),
                    // Load library with `RTLD_NOW | RTLD_NODELETE` to fix SIGSEGV.
                    0x2 | 0x1000,
                )
                .map(|library| libloading::Library::from(library))
            }

            #[cfg(target_os = "windows")]
            {
                libloading::Library::new(path.as_ref())
            }
        }
        .map_err(|error| {
            LoaderError::Loading(format!(
                "failed to load library at {:?}: {}",
                path.as_ref(),
                error
            ))
        })?;

        self.libraries.push(library);
        let library = self.libraries.last().expect("library just got inserted");

        let module_entrypoint = unsafe { library.get::<Entrypoint>(MODULE_ENTRYPOINT.as_bytes()) }
            // TODO: make this error message more explicit.
            .map_err(|error| LoaderError::Loading(error.to_string()))?;

        Ok(module_entrypoint())
    }
}
