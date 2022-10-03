pub(crate) type PluginConstructor = fn() -> Box<dyn Plugin>;

pub(crate) const PLUGIN_ENTRYPOINT: &str = "plugin_entrypoint";

/// Trait used to register new rhai modules from a dynamic library.
pub trait Plugin {
    /// Update a rhai engine with whatever API the plugin exposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// pub struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn register(&self, engine: &mut rhai::Engine) {
    ///         // register a new function into the engine.
    ///         engine.register_fn("my_plugin_function", || println!("Hello from a plugin !"));
    ///
    ///         // modify the engine's configuration.
    ///         engine.disable_symbol("if");
    ///
    ///         // configure something else ...
    ///     }
    /// }
    /// ```
    ///
    /// A really nice way to implement a plugin is using Rhai's [plugin modules](https://rhai.rs/book/plugins/module.html).
    ///
    /// ```rust
    /// /// Use the `export_module` macro to generate your api.
    /// #[rhai::export_module]
    /// mod my_api {
    ///     pub fn get_num() -> i64 {
    ///         3
    ///     }
    ///     pub fn print_stuff() {
    ///        println!("Hello World!");
    ///     }
    /// }
    ///
    /// pub struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn register(&self, engine: &mut rhai::Engine) {
    ///         // register the module into the engine.
    ///         // `get_num` & `print_stuff` will be available globally in the engine !
    ///         engine.register_global_module(rhai::exported_module!(my_api).into());
    ///
    ///         // You could also use `register_static_module` ... or anything the engine provides.
    ///     }
    /// }
    /// ```
    ///
    ///
    fn register(&self, engine: &mut rhai::Engine);
}
