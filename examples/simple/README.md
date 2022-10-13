# Simple

Building a plugin as a dylib and loading it using the dylib plugin loader.

The `plugin` crate contains all of our plugin code.
The `main.rs` file creates a loader using [`libloading`](https://github.com/nagisa/rust_libloading), load the plugin and run some of it's api.

Run the example:

```sh
cd plugin && cargo build && cd ..
cargo run
```