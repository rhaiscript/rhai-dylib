pub(crate) type PluginConstructor = fn() -> Box<dyn Plugin>;

pub(crate) const PLUGIN_ENTRYPOINT: &str = "plugin_entrypoint";

/// Trait used to register new rhai modules from a dynamic library.
pub trait Plugin {
    fn register(&self, builder: Builder);
}

/// A builder used to register the rhai api of a plugin. (prevent the user to disable keywords and what not.)
pub struct Builder<'re> {
    engine: &'re mut rhai::Engine,
}

impl<'re> Builder<'re> {
    pub fn new(engine: &'re mut rhai::Engine) -> Self {
        Self { engine }
    }

    /// Add a module to the rhai context of vSMTP.
    pub fn register_global_module(&mut self, module: rhai::Module) -> &mut Self {
        self.engine.register_global_module(module.into());

        self
    }
}
