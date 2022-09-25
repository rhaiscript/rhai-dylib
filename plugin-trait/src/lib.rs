type PluginConstructor = fn() -> Box<dyn Plugin>;

#[cfg(target_os = "linux")]
const PATH_TO_PLUGIN: &str = "./target/debug/libplugin.so"; // "../plugin/target/debug/libplugin.so";
#[cfg(target_os = "windows")]
const PATH_TO_PLUGIN: &str = "./target/debug/libplugin.dll";

pub const PLUGIN_ENTRYPOINT: &str = "plugin_entrypoint";

pub struct Loader {
    pub plugins: Vec<Box<dyn Plugin>>,
    pub libraries: Vec<libloading::Library>,
}

impl Default for Loader {
    fn default() -> Self {
        println!("trait: {:?}", std::any::TypeId::of::<rhai::Map>());

        let mut loader = Self {
            plugins: vec![],
            libraries: vec![],
        };

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

        loader.libraries.push(library);
        let library = loader.libraries.last().unwrap();

        let constructor = unsafe { library.get::<PluginConstructor>(PLUGIN_ENTRYPOINT.as_bytes()) }
            .expect("failed to load entrypoint symbol");

        let plugin = constructor();

        loader.plugins.push(plugin);

        loader
    }
}

/// Trait used to register new rhai modules from a dynamic library.
pub trait Plugin {
    fn register(&self, builder: Builder);
}

/// A builder used to register the rhai api of a plugin. (prevent the user to disable keywords and what not.)
pub struct Builder<'re> {
    engine: &'re mut rhai::Engine,
}

impl<'re> Builder<'re> {
    pub fn new(engine: &'re mut rhai::Engine) -> Self {
        Self { engine }
    }

    /// Add a module to the rhai context of vSMTP.
    pub fn register_global_module(&mut self, module: rhai::Module) -> &mut Self {
        self.engine.register_global_module(module.into());

        self
    }
}
