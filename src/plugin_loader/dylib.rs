use crate::plugin::Plugin;

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
    /// Plugins loaded in memory.
    plugins: Vec<Box<dyn Plugin>>,
    /// Libraries loaded in memory.
    libraries: Vec<libloading::Library>,
}

impl Default for Libloading {
    /// Create a new instance of the loader.
    fn default() -> Self {
        Self {
            plugins: vec![],
            libraries: vec![],
        }
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
    #[cfg(feature = "c")]
    fn load<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<&'a Box<dyn Plugin>, PluginLoaderError> {
        use crate::plugin::{CEntrypointPrototype, C_PLUGIN_ENTRYPOINT};

        self.load_library_inner(path)?;

        let library = self.libraries.last().expect("library just got inserted");

        let constructor =
            unsafe { library.get::<CEntrypointPrototype>(C_PLUGIN_ENTRYPOINT.as_bytes()) }
                // TODO: make this error message more explicit.
                .map_err(|error| PluginLoaderError::Loading(error.to_string()))?;

        let plugin = constructor();

        self.plugins.push(plugin.0);

        Ok(self.plugins.last().expect("plugin just got inserted"))
    }

    #[cfg(feature = "rust")]
    fn load_rust_unstable<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<&'a Box<dyn crate::plugin::Plugin>, PluginLoaderError> {
        use crate::plugin::{RustEntrypointPrototype, RUST_PLUGIN_ENTRYPOINT};

        self.load_library_inner(path)?;

        let library = self.libraries.last().expect("library just got inserted");

        let constructor =
            unsafe { library.get::<RustEntrypointPrototype>(RUST_PLUGIN_ENTRYPOINT.as_bytes()) }
                // TODO: make this error message more explicit.
                .map_err(|error| PluginLoaderError::Loading(error.to_string()))?;

        let plugin = constructor();

        self.plugins.push(plugin);

        Ok(self.plugins.last().expect("plugin just got inserted"))
    }

    #[cfg(feature = "abi-stable")]
    fn load_rust_stable<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<&'a Box<dyn crate::plugin::Plugin>, PluginLoaderError> {
        todo!("abi-stable not implemented yet. {:?}", path.as_ref())
    }

    /// Apply all plugins loaded via the [`Libloading::load`] method to a rhai engine.
    fn apply(&self, engine: &mut rhai::Engine) -> Result<(), super::PluginLoaderError> {
        self.plugins
            .iter()
            .for_each(|plugin| plugin.register(engine));

        Ok(())
    }
}

impl Libloading {
    fn load_library_inner<'a>(
        &'a mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<&'a Box<dyn crate::plugin::Plugin>, PluginLoaderError> {
        let library = unsafe {
            if cfg!(linux) {
                // Workaround for a crash on library unloading on linux: https://github.com/nagisa/rust_libloading/issues/5#issuecomment-244195096
                libloading::os::unix::Library::open(
                    Some(path.as_ref()),
                    // Load library with `RTLD_NOW | RTLD_NODELETE` to fix SIGSEGV.
                    0x2 | 0x1000,
                )
                .map(|library| libloading::Library::from(library))
            } else if cfg!(windows) {
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

        todo!()
    }
}
