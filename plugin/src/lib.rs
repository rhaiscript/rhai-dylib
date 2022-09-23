pub mod api;

struct MyPlugin;

impl plugin_trait::Plugin for MyPlugin {
    fn register(&self, mut builder: plugin_trait::Builder) {
        println!("Plugin: {:?}", std::any::TypeId::of::<rhai::Map>());
        builder.register_global_module(rhai::exported_module!(api::my_plugin_api));
    }
}

#[no_mangle]
pub fn plugin_entrypoint() -> Box<dyn plugin_trait::Plugin> {
    Box::new(MyPlugin {})
}
