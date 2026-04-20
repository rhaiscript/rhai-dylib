use rhai_dylib::rhai::{config::hashing::set_hashing_seed, Module, Shared, INT};

// A really simple plugin used as a real dynamic library in unit tests.
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn module_entrypoint() -> Shared<Module> {
    let _ = set_hashing_seed(Some([1, 2, 3, 4]));
    let mut module = Module::new();

    module.set_native_fn("add", |a: INT, b: INT| Ok(a + b));
    module.into()
}
