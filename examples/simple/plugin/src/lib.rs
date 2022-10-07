pub mod api;

#[no_mangle]
pub extern "C" fn register_rhai_plugin(engine: &mut rhai_dylib::rhai::Engine) {
    // Checking if TypeIDs are the same as the main program.
    println!(
        "plugin: {:?}",
        std::any::TypeId::of::<rhai_dylib::rhai::Map>()
    );

    engine.register_global_module(rhai_dylib::rhai::exported_module!(api::my_plugin_api).into());
}

// #[no_mangle]
// pub fn rust_plugin_entrypoint() -> Box<dyn Plugin> {
//     Box::new(MyPlugin {})
// }
