# Module Resolver

Building a plugin as a dylib and loading it using the `DylibModuleResolver`.

The `plugin` crate contains all of our plugin code.
The `main.rs` file creates a Rhai engine and run a script with an import statement pointing to the `plugin` crate dylib.

Run the example:

```sh
cd plugin && cargo build && cd ..
cargo run
```