pub(crate) type PluginConstructor = fn() -> Box<dyn Plugin>;

pub(crate) const PLUGIN_ENTRYPOINT: &str = "plugin_entrypoint";

/// Trait used to register new rhai modules from a dynamic library.
pub trait Plugin {
    /// Update a rhai engine with whatever API the plugin exposes.
    fn register(&self, engine: &mut rhai::Engine);
}
