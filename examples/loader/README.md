# Simple

Building a plugin as a dylib and loading it using the dylib plugin loader.

The `main.rs` file creates a loader using [`libloading`](https://github.com/nagisa/rust_libloading), load the plugin and run some of it's api.

Run the example:

```sh
cargo build --example dynamic_library
cargo run --example loader
```