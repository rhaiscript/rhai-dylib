use super::{PluginLoader, PluginLoaderError};

/// Loading dynamic libraries using the [`libloading`](https://github.com/nagisa/rust_libloading) crate.
///
/// # Example
///
/// ```rust
/// // Create your dynamic library loader.
/// let mut plugin_registry = Libloading::new();
///
/// // `my_first_plugin` library exposes the `print_first` function.
/// plugin_registry.load("my_first_plugin.so").expect("failed to load library 1");
/// // `my_second_plugin` library exposes the `print_second` function.
/// plugin_registry.load("my_second_plugin.so").expect("failed to load library 2");
///
/// let mut engine = rhai::Engine::new();
///
/// // Apply both plugins to the engine.
/// plugin_registry.apply(&mut engine);
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

impl PluginLoader for Libloading {
    /// Load a plugin from a dynamic library.
    fn load<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
        engine: &mut rhai::Engine,
    ) -> Result<(), PluginLoaderError> {
        use crate::plugin::{Entrypoint, PLUGIN_ENTRYPOINT};

        let library = unsafe {
            if cfg!(target_os = "linux") {
                // Workaround for a crash on library unloading on linux: https://github.com/nagisa/rust_libloading/issues/5#issuecomment-244195096
                libloading::os::unix::Library::open(
                    Some(path.as_ref()),
                    // Load library with `RTLD_NOW | RTLD_NODELETE` to fix SIGSEGV.
                    0x2 | 0x1000,
                )
                .map(|library| libloading::Library::from(library))
            } else if cfg!(target_os = "windows") {
                libloading::Library::new(path.as_ref())
            } else {
                todo!("unsupported platform, available are linux & windows")
            }
        }
        .map_err(|error| {
            PluginLoaderError::Loading(format!(
                "failed to load library at {:?}: {}",
                path.as_ref(),
                error
            ))
        })?;

        self.libraries.push(library);
        let library = self.libraries.last().expect("library just got inserted");

        let register = unsafe { library.get::<Entrypoint>(PLUGIN_ENTRYPOINT.as_bytes()) }
            // TODO: make this error message more explicit.
            .map_err(|error| PluginLoaderError::Loading(error.to_string()))?;

        register(engine);

        Ok(())
    }
}
