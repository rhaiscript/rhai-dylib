use crate::plugin::{Builder, Plugin, PluginConstructor, PLUGIN_ENTRYPOINT};

use super::{PluginLoader, PluginLoaderError};

/// An object used to load and store plugins for Rhai.
pub struct Libloading {
    /// Plugins loaded in memory.
    pub plugins: Vec<Box<dyn Plugin>>,
    /// Libraries loaded in memory.
    pub libraries: Vec<libloading::Library>,
}

impl Default for Libloading {
    fn default() -> Self {
        Self {
            plugins: vec![],
            libraries: vec![],
        }
    }
}

impl Libloading {
    /// Create a new plugin loader using libloading.
    pub fn new() -> Self {
        Self::default()
    }
}

impl PluginLoader for Libloading {
    /// Load a plugin from a dynamic library.
    fn load<'a>(
        &'a mut self,
        path: &'_ std::path::Path,
    ) -> Result<&'a Box<dyn Plugin>, PluginLoaderError> {
        let library = unsafe {
            // TODO: add Macos support. (and other platforms ?)
            if cfg!(linux) {
                // Workaround for a crash on library unloading on linux: https://github.com/nagisa/rust_libloading/issues/5#issuecomment-244195096
                libloading::os::unix::Library::open(
                    Some(path),
                    // Load library with `RTLD_NOW | RTLD_NODELETE` to fix SIGSEGV.
                    0x2 | 0x1000,
                )
                .map(|library| libloading::Library::from(library))
            } else {
                libloading::Library::new(path)
            }
        }
        .map_err(|error| {
            PluginLoaderError::Loading(format!("failed to load library at {path:?}: {error}"))
        })?;

        self.libraries.push(library);
        let library = self.libraries.last().unwrap();

        let constructor = unsafe { library.get::<PluginConstructor>(PLUGIN_ENTRYPOINT.as_bytes()) }
            .expect("failed to load entrypoint symbol");

        let plugin = constructor();

        self.plugins.push(plugin);

        Ok(self.plugins.last().unwrap())
    }

    /// Apply all plugins loaded in memory to a rhai engine.
    fn apply(&mut self, engine: &mut rhai::Engine) -> Result<(), super::PluginLoaderError> {
        for plugin in &self.plugins {
            plugin.register(Builder::new(engine));
        }

        Ok(())
    }
}
