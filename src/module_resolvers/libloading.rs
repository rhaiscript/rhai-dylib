//!

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "windows"))]
compile_error!("unsupported platform - only Linux & Windows are supported");

use std::str::FromStr;

use crate::loader::{libloading::Libloading, Loader};

/// A module resolver that load dynamic libraries pointed by the `import` path.
pub struct DylibModuleResolver {
    /// Path prepended for each import if specified.
    base_path: Option<std::path::PathBuf>,
    /// Dynamic library loader.
    loader: rhai::Locked<Libloading>,
    /// Is module caching enabled for this resolver.
    cache_enabled: bool,
    /// Cache of loaded modules, empty if [`Self::cache_enabled`] is false.
    cache: rhai::Locked<std::collections::BTreeMap<std::path::PathBuf, rhai::Shared<rhai::Module>>>,
}

impl Default for DylibModuleResolver {
    fn default() -> Self {
        Self {
            base_path: None,
            loader: Libloading::new().into(),
            cache_enabled: true,
            cache: rhai::Locked::new(std::collections::BTreeMap::new()),
        }
    }
}

impl DylibModuleResolver {
    /// Create a new instance of the resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable/disable the cache.
    #[inline(always)]
    pub fn enable_cache(&mut self, enable: bool) -> &mut Self {
        self.cache_enabled = enable;
        self
    }

    /// Is the cache enabled?
    #[inline(always)]
    #[must_use]
    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
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
    /// // from the 'scripts' directory.
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
}

impl rhai::ModuleResolver for DylibModuleResolver {
    fn resolve(
        &self,
        _: &rhai::Engine,
        source: Option<&str>,
        path: &str,
        pos: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        dbg!(source, path, pos);

        let path = source
            .map(|source| std::path::PathBuf::from_str(source).expect("is infallible"))
            .unwrap_or_else(|| std::path::PathBuf::from_str(path).expect("is infallible"));

        let mut path = self
            .base_path
            .as_ref()
            .and_then(|base_path| Some(std::path::PathBuf::from_iter([base_path, &path])))
            .unwrap_or(path);

        #[cfg(target_os = "linux")]
        path.set_extension("so");
        #[cfg(target_os = "windows")]
        path.set_extension("dll");

        // NOTE: check for rhai's `locked_read` & `locked_write` methods.
        let mut cache = self.cache.borrow_mut();

        let load_module = || {
            self.loader
                .borrow_mut()
                .load(path.as_path())
                .map_err(|err| err.into())
        };

        if !self.is_cache_enabled() {
            load_module()
        } else if let Some(module) = cache.get(&path) {
            Ok(module.clone())
        } else {
            let module = load_module()?;
            cache.insert(path, module.clone());

            Ok(module)
        }
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
