pub mod api;

use rhai_dylib::rhai::{exported_module, Module, Shared};

#[no_mangle]
pub extern "C" fn module_entrypoint() -> Shared<Module> {
    // Checking if TypeIDs are the same as the main program.
    println!(
        "plugin: {:?}",
        std::any::TypeId::of::<rhai_dylib::rhai::Map>()
    );

    exported_module!(api::my_plugin_api).into()
}
