//!

use std::str::FromStr;

use crate::loader::{dylib::Libloading, Loader};

/// A module resolver that load dynamic libraries pointed by the `import` path.
pub struct DylibModuleResolver {
    /// Path prepended for each import if specified.
    base_path: Option<std::path::PathBuf>,
    /// Dynamic library loader.
    loader: rhai::Locked<Libloading>,
    // cache_enabled: bool,
    // cache: Locked<BTreeMap<PathBuf, Shared<Module>>>,
}

impl Default for DylibModuleResolver {
    fn default() -> Self {
        Self {
            base_path: None,
            loader: Libloading::new().into(),
        }
    }
}

impl DylibModuleResolver {
    /// Create a new instance of the resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`DylibModuleResolver`] with a specific base path.
    ///
    /// # Example
    ///
    /// ```
    /// use rhai::Engine;
    /// use rhai_dylib::module_resolvers::DylibModuleResolver;
    ///
    /// // Create a new 'DylibModuleResolver' loading dynamic libraries
    /// // from the 'scripts' subdirectory.
    /// let resolver = DylibModuleResolver::with_path("./scripts");
    ///
    /// let mut engine = Engine::new();
    /// engine.set_module_resolver(resolver);
    /// ```
    #[must_use]
    pub fn with_path(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            base_path: Some(path.into()),
            ..Default::default()
        }
    }

    // /// Construct a path to the desired dynamic library.
    // #[must_use]
    // fn get_file_path(
    //     &self,
    //     path: &str,
    //     source_path: Option<&std::path::Path>,
    // ) -> std::path::PathBuf {
    //     let path = std::path::Path::new(path);

    //     let mut file_path;

    //     if path.is_relative() {
    //         file_path = self
    //             .base_path
    //             .clone()
    //             .or_else(|| source_path.map(Into::into))
    //             .unwrap_or_default();
    //         file_path.push(path);
    //     } else {
    //         file_path = path.into();
    //     }

    //     #[cfg(target_os = "linux")]
    //     file_path.set_extension("so");
    //     #[cfg(target_os = "windows")]
    //     file_path.set_extension("dll");

    //     file_path
    // }
}

impl rhai::ModuleResolver for DylibModuleResolver {
    fn resolve(
        &self,
        engine: &rhai::Engine,
        source: Option<&str>,
        path: &str,
        pos: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        dbg!(source, path, pos);

        let mut path = source
            .map(|source| std::path::PathBuf::from_str(source).expect("is infallible"))
            .unwrap_or_else(|| std::path::PathBuf::from_str(path).expect("is infallible"));

        #[cfg(target_os = "linux")]
        path.set_extension("so");
        #[cfg(target_os = "windows")]
        path.set_extension("dll");

        dbg!(&path);

        self.loader
            .borrow_mut()
            .load(path.as_path())
            .map_err(|err| err.into())
    }

    fn resolve_raw(
        &self,
        engine: &rhai::Engine,
        global: &mut rhai::GlobalRuntimeState,
        path: &str,
        pos: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        self.resolve(engine, global.source(), path, pos)
    }

    fn resolve_ast(
        &self,
        _: &rhai::Engine,
        _: Option<&str>,
        _: &str,
        _: rhai::Position,
    ) -> Option<Result<rhai::AST, Box<rhai::EvalAltResult>>> {
        // This resolver is Rust based, so it cannot resolve ASTs.
        None
    }
}
