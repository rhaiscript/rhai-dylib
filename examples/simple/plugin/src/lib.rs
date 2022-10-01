pub mod api;

use rhai_dylib::plugin::Plugin;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(&self, engine: &mut rhai_dylib::rhai::Engine) {
        // Checking if TypeIDs are the same as the main program.
        println!(
            "plugin: {:?}",
            std::any::TypeId::of::<rhai_dylib::rhai::Map>()
        );

        engine
            .register_global_module(rhai_dylib::rhai::exported_module!(api::my_plugin_api).into());
    }
}

#[no_mangle]
pub fn plugin_entrypoint() -> Box<dyn Plugin> {
    Box::new(MyPlugin {})
}
