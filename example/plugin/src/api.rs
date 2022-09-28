use rhai::plugin::{
    mem, Dynamic, FnAccess, FnNamespace, NativeCallContext, PluginFunction, RhaiResult, TypeId,
};

use rhai::Module;

#[rhai::plugin::export_module]
pub mod my_plugin_api {

    /// This function does not take any parameters
    #[rhai_fn(global)]
    pub fn no_params() {
        println!("this is a test function");
    }

    /// This function takes a map as parameter.
    #[rhai_fn(global)]
    pub fn with_params(parameters: rhai::Map) {
        println!("Fn with a map: {parameters:?}");
    }
}
