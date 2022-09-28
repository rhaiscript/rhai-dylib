pub mod dylib;

pub enum PluginLoaderError {
    Loading(String),
}

/// A trait to implement an object that stores plugins.
pub trait PluginLoader {
    /// Load a plugin from a path.
    fn load<'a>(
        &'a mut self,
        path: &'_ std::path::Path,
    ) -> Result<&'a Box<dyn crate::plugin::Plugin>, PluginLoaderError>;

    /// Apply plugins to a rule engine.
    fn apply(&mut self, engine: &mut rhai::Engine) -> Result<(), PluginLoaderError>;
}
