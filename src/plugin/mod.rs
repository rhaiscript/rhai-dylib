//! # Creating a plugin.
//!
//! You need to declare the entrypoint function of your plugin, following the [`Entrypoint`] prototype.
//! The name of the function must be the same as [`PLUGIN_ENTRYPOINT`].
//!
//! ```rust
//! fn register_rhai_plugin(engine: &mut rhai::Engine) {
//!     // ...
//! }
//! ```
//!
//! A really nice way to implement a plugin is using Rhai's [plugin modules](https://rhai.rs/book/plugins/module.html).
//!
//! ```rust
//! // Use the `export_module` macro to generate your api.
//! #[rhai::export_module]
//! mod my_api {
//!     pub fn get_num() -> i64 {
//!         3
//!     }
//!     pub fn print_stuff() {
//!        println!("Hello World!");
//!     }
//! }
//!
//! // The entrypoint function of your plugin.
//! // `extern "C"` can be omitted if you are using the `rust` feature.
//! #[no_mangle]
//! extern "C" fn register_rhai_plugin(engine: &mut rhai::Engine) {
//!     // register the module into the engine.
//!     // `get_num` & `print_stuff` will be available globally in the engine !
//!     engine.register_global_module(rhai::exported_module!(my_api).into());
//!
//!     // You could also use `register_static_module` ... or anything the engine provides !
//! }
//! ```

/// Entrypoint prototype for a plugin.
pub type Entrypoint = fn(&mut rhai::Engine);
/// The name of the function that will be called to register the plugin.
pub const PLUGIN_ENTRYPOINT: &str = "register_rhai_plugin";
