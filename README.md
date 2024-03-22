# Rhai Dylib

This crate exposes a simple API to load `dylib` Rust crates in a [Rhai](https://rhai.rs/) engine using [Rhai modules](https://rhai.rs/book/rust/modules/index.html).
You can generate your own library project for [Rhai](https://rhai.rs/) using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) with [`this template`](https://github.com/ltabis/rhai-dylib-template).

## Loader

`Loader` is a trait that is used to build objects that load rhai modules from dynamic libraries in memory. A [libloading](https://github.com/nagisa/rust_libloading) implementation is available, which enables you to load modules via a `cdylib` or `dylib` rust crate.

Check the `simple` example for more details.

> You can easily setup a dynamic library for Rhai by using [cargo-generate](https://github.com/cargo-generate/cargo-generate) and the [rhai-dylib-template](https://github.com/ltabis/rhai-dylib-template).

## Module Resolver

This crate also expose a [Rhai Module Resolver](https://rhai.rs/book/rust/modules/resolvers.html) that loads dynamic libraries at the given path.

```rust,ignore
use rhai_dylib::module_resolvers::libloading::DylibModuleResolver;

let mut engine = rhai::Engine::new();

// use `rhai::module_resolvers::ModuleResolversCollection` if you need to resolve using
// other resolvers.
// Check out https://docs.rs/rhai/latest/rhai/module_resolvers/struct.ModuleResolversCollection.html
engine.set_module_resolver(DylibModuleResolver::new());

engine.run(r#"
import "/usr/lib/libmy" as my; // Import your dynamic library.

my::my_function(); // Use exported items !
"#).expect("failed to run script");
```

Check the `module_resolver` example for more details.

## Pitfalls

There are multiple limitations with this implementation.

> TL;DR
> To use this crate, you need to:
> - Compile **EVERYTHING**, plugins and program that will load them, inside the **SAME** workspace or **WITHOUT** a workspace.
> - Use the `rhai::config::hashing::set_hashing_seed` function with the **SAME** four u64 array when building your plugins and the program that will load them. (i.e. `rhai::config::hashing::set_hashing_seed(Some([1, 2, 3, 4]))`)

### TypeId

Rust [`TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html) is an object used to compare types at compile time. Rhai uses those to check which type a [`Dynamic`](https://docs.rs/rhai/1.10.1/rhai/struct.Dynamic.html) object is. This is a problem for dynamic libraries because `TypeIds` sometime change between compilations.

That means that in certain situations, Rhai cannot compare two types, even though they are the same, because the `TypeId` of said types is different between the plugin and the binary.

To fix this, you will need to compile your main binary **AND** plugins inside the **SAME** workspace, or compile everything **OUTSIDE** of a workspace. Compiling, for example, a binary in a workspace, and a plugin outside will probably result in `TypeIds` mismatch.

> You can use
> ```rust,ignore
> println!("{:?}", std::any::TypeId::of::<rhai::Map>());
> ```
> In your binary & plugins to check the type id value.

If you have any idea of how the compiler generates those typeids between workspaces and single crates, please help us complete this readme !

### Hashing

Rhai uses the [`ahash`](https://github.com/tkaitchuck/ahash) crate under the hood to create identifiers for function calls. For each compilation of your code, a new seed is generated when hashing the types. Therefore, compiling your main program and your plugin different times will result in a hash mismatch, meaning that you won't be able to call the API of your plugin.

To bypass that, you need to use the `rhai::config::hashing::set_hashing_seed` function with an array of four `u64`.

### Others

If your plugin is still not working after you read the sections above, you could check those points too:

- Use the same toolchain version for all crates.
- Use the same rhai-dylib dependency version in all crates.
- Try to build the crates with the same target type. (both debug or both release)
- Enable/Disable link time optimization flag (-C lto) for all crates.

If none of those solutions works, do not hesitate to open an issue !

## Rust ABI

You also can implement a plugin using the Rust ABI, which is unstable and will change between compiler versions.

This means that all of the plugins that you will use in your main program need to be compiled with the **EXACT** same
compiler version.