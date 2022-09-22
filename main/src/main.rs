type PluginConstructor = fn() -> Box<dyn plugin_trait::Plugin>;

#[cfg(target_os = "linux")]
const PATH_TO_PLUGIN: &str = "./target/debug/libplugin.so";
#[cfg(target_os = "windows")]
const PATH_TO_PLUGIN: &str = "./target/debug/libplugin.dll";

pub const PLUGIN_ENTRYPOINT: &str = "plugin_entrypoint";

fn main() {
    let library = unsafe {
        // Workaround for a crash on library unloading on linux: https://github.com/nagisa/rust_libloading/issues/5#issuecomment-244195096
        if cfg!(linux) {
            libloading::Library::from(
                libloading::os::unix::Library::open(
                    Some(PATH_TO_PLUGIN),
                    // Load library with `RTLD_NOW | RTLD_NODELETE` to fix SIGSEGV.
                    0x2 | 0x1000,
                )
                .expect("failed to open dylib"),
            )
        } else {
            libloading::Library::new(PATH_TO_PLUGIN).expect("failed to open dylib")
        }
    };

    let constructor = unsafe { library.get::<PluginConstructor>(PLUGIN_ENTRYPOINT.as_bytes()) }
        .expect("failed to load entrypoint symbol");

    let plugin = constructor();

    let mut loader = plugin_trait::LibEngine::new();

    // Register the plugin's module into the engine.
    plugin.register(plugin_trait::Builder::new(&mut loader.engine));

    // checking if the module has been registered.
    println!("{:#?}", loader.engine.gen_fn_signatures(false));

    loader.engine.run("with_params(#{});").unwrap();
}
