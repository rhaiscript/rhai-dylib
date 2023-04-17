# Module Resolver

Building a plugin as a dylib and loading it using the `DylibModuleResolver`.

The `main.rs` file creates a Rhai engine and run a script with an import statement pointing to the `plugin` crate dylib.

Run the example:

```sh
cargo build --example dynamic_library
cargo run --example module_resolver
```
