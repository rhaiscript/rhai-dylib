pub mod api;

use rhai_dylib::rhai::{exported_module, Module, Shared};

#[no_mangle]
pub extern "C" fn module_entrypoint() -> Shared<Module> {
    exported_module!(api::my_plugin_api).into()
}
