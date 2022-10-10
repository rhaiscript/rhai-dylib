use rhai_dylib::rhai::plugin::{
    mem, Dynamic, FnAccess, FnNamespace, NativeCallContext, PluginFunction, RhaiResult, TypeId,
};

use rhai_dylib::rhai::{Module, INT};

#[rhai_dylib::rhai::plugin::export_module]
pub mod my_plugin_api {

    // The plugin API from rhai can be used to create your plugin API.

    /// Printing to the console using Rust.
    // #[rhai_fn(global)]
    pub fn print_stuff() {
        println!("Hello from plugin!");
    }

    /// Computing something and returning a result.
    // #[rhai_fn(global)]
    pub fn triple_add(a: INT, b: INT, c: INT) -> INT {
        a + b + c
    }
}
