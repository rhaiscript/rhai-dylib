# Rhai Dylib

This crate exposes a simple API to create "plugins" using native rust modules and functions to register in a Rhai engine.
Plugins can only be loaded in the form of dynamic libraries for now.

> 🚧 This is a work in progress, the API is subject to change. Please do make recommendations on what you want it to be via issues, discussions or pull requests !

## Plugins

`Plugin` is a simple trait that exposes a `register` function. An instance of a rhai engine is passed to this function to enable the plugin to change the engine's parameters, register new function, modules etc ...

## Loader

`PluginLoader` is a trait that is used to build objects that load plugins and apply them via the `register` function to a given engine instance. A [libloading](https://github.com/nagisa/rust_libloading) implementation is available, which enables you to load plugins via a `dylib` rust crate.

## Pitfalls

There are multiple limitations with this implementation.

> TL;DR
> - Compile **EVERYTHING**, plugins and program that will load them, inside the **SAME** workspace or **WITHOUT** a workspace.
> - Export the `RHAI_AHASH_SEED` environment variable with the **SAME** four u64 array (i.e. `RHAI_AHASH_SEED="[1, 2, 3, 4]"`) when building your plugins and the program that will load them.

### TypeId

Rust [`TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html) is an object used to compare types at compile time. Rhai uses those to check which type a [`Dynamic`](https://docs.rs/rhai/1.10.1/rhai/struct.Dynamic.html) object is. This is a problem for dynamic libraries because `TypeIds` sometime change between compilations.

That means that in certain situations, Rhai cannot compare two types, even tough they are the same, because the `TypeId` of said types is different between the plugin and the binary.

To fix this, you will need to compile your main binary **AND** plugins inside the **SAME** workspace, or compile everything **OUTSIDE** of a workspace. Compiling, for example, a binary in a workspace, and a plugin outside will probably result in `TypeIds` mismatch.

>  You can use
> ```rust
> println!("{:?}", std::any::TypeId::of::<rhai::Map>());
> ```
> In your binary & plugins to check the type id value.

If you have any idea of how the compiler generates those typeids between workspaces and single crates, please help us complete this readme !

### Hashing

Rhai uses ahash under the hood to create identifiers for function calls. For each compilation of your code, a new seed is generated when hashing the types. Meaning that compiling your main program and your plugin different times will result in a hash mismatch, meaning that you won't be able to call the API of your plugin.

To bypass that, you need to inject the `RHAI_AHASH_SEED` environment variable with an array of four `u64`.

```sh
export RHAI_AHASH_SEED="[1, 2, 3, 4]" # The seed is now fixed and won't change between compilations.

# Compiling will create the same hashes for functions.
cargo build --manifest-path ./my_program/Cargo.toml
cargo build --manifest-path ./my_plugin/Cargo.toml
```

## TODO

Here is a list of stuff that we could implement or think about. (to move in issues)

- [ ] How could we "restrain" the API access of the rhai engine ? Lock those behind features ? Using a new type that wraps the engine ?
- [ ] What ABI should be used ? Should we lock different ABIs behind features ?
- [ ] Configure libloading for multiple targets.
- [ ] Lock plugin loaders behind features.
- [ ] Create macros that generate entry points.
- [ ] Update seeds for ahash.
- [ ] Add examples.
- [ ] Add some unit & integration tests.