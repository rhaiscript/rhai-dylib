pub mod api;

use rhai_dylib::rhai::{
    config::hashing::set_ahash_seed, exported_module, ImmutableString, Module, Shared,
};

#[no_mangle]
pub extern "C" fn module_entrypoint() -> Shared<Module> {
    set_ahash_seed(Some([1, 2, 3, 4])).unwrap();

    // Checking if TypeIDs are the same as the main program.
    println!("plugin: {:?}", std::any::TypeId::of::<ImmutableString>());

    exported_module!(api::my_plugin_api).into()
}
