use rhai_dylib::rhai::plugin::{
    mem, Dynamic, FnAccess, FnNamespace, ImmutableString, NativeCallContext, PluginFunction,
    RhaiResult, TypeId,
};

use rhai_dylib::rhai::{Map, Module, INT};

#[derive(Clone)]
pub struct MyPluginObjectInner {
    inner: String,
}

// The plugin API from rhai can be used to create your plugin API.
#[rhai_dylib::rhai::plugin::export_module]
pub mod my_plugin_api {

    // Implement a custom type.
    type MyPluginObject = MyPluginObjectInner;

    // Constructor for the custom type.
    #[rhai_fn(global)]
    pub fn new_plugin_object(inner: &str) -> MyPluginObject {
        MyPluginObject {
            inner: inner.to_string(),
        }
    }

    /// A function for the custom type.
    #[rhai_fn(global)]
    pub fn display_inner(s: &mut MyPluginObject) {
        println!("{}", s.inner);
    }

    /// Printing to the console using Rust.
    #[rhai_fn(global)]
    pub fn print_stuff() {
        println!("Hello from plugin!");
    }

    /// Computing something and returning a result.
    #[rhai_fn(global)]
    pub fn triple_add(a: INT, b: INT, c: INT) -> INT {
        a + b + c
    }

    /// Using Rhai types.
    #[rhai_fn(global)]
    pub fn get_property(m: &mut Map) -> String {
        m.get("property").unwrap().clone_cast()
    }
}
