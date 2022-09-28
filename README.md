# Rhai Native plugins

This crate exposes an API to create "plugins" using native rust modules and functions to register in a Rhai engine.

## Plugins

`Plugin` is a simple trait that exposes a `register` function. An instance of a rhai engine is passed to this function to enable the plugin to change the engine's parameters, register new function, modules etc ...

## Loader

`PluginLoader` is a trait that is used to build objects that load plugins and apply them via the `register` function to a given engine instance. A [libloading](https://github.com/nagisa/rust_libloading) implementation is available, which enables you to load plugins via a dylib.

# TODO

Here is a list of stuff that we could implement or think about. (to move in issues)

- [ ] Should we "restrain" the access to all of the engines functions ? Lock those behind features ?
- [ ] What ABI should be used ? Should we lock different ABIs behind features ?
- [ ] Configure libloading for multiple targets.
- [ ] Lock plugin loaders behind features.
- [ ] Create macros that generate entry points.
- [ ] Add TypeID pitfall and ahash specification.
- [ ] Update seeds for ahash.
- [ ] Add examples
- [ ] Add some unit & integration tests.