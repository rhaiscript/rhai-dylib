use super::{locked_read, locked_write};
use crate::loader::libloading::Libloading;
use crate::loader::Loader;

#[cfg(target_os = "linux")]
const DYLIB_EXTENSION: &str = "so";
#[cfg(target_os = "macos")]
const DYLIB_EXTENSION: &str = "dylib";
#[cfg(target_os = "windows")]
const DYLIB_EXTENSION: &str = "dll";

/// A module resolver that load dynamic libraries pointed by the `import` path.
pub struct DylibModuleResolver {
    /// Path prepended for each import if specified.
    base_path: Option<std::path::PathBuf>,
    /// Is module caching enabled for this resolver.
    cache_enabled: bool,
    /// Cache of loaded modules, empty if [`Self::cache_enabled`] is false.
    cache: rhai::Locked<std::collections::BTreeMap<std::path::PathBuf, rhai::Shared<rhai::Module>>>,
    /// Dynamic library loader.
    loader: rhai::Locked<Libloading>,
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable/disable the cache.
    pub fn enable_cache(&mut self, enable: bool) -> &mut Self {
        self.cache_enabled = enable;
        self
    }

    /// Is the cache enabled?
    #[must_use]
    pub const fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    /// Create a new [`DylibModuleResolver`] with a specific base path.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rhai::Engine;
    /// use rhai_dylib::module_resolvers::libloading::DylibModuleResolver;
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

    /// Construct a full file path.
    #[must_use]
    pub fn get_file_path(
        &self,
        path: &str,
        source_path: Option<&std::path::Path>,
    ) -> std::path::PathBuf {
        let path = std::path::Path::new(path);

        let mut file_path;

        if path.is_relative() {
            file_path = self
                .base_path
                .clone()
                .or_else(|| source_path.map(Into::into))
                .unwrap_or_default();
            file_path.push(path);
        } else {
            file_path = path.into();
        }

        file_path.set_extension(DYLIB_EXTENSION);

        file_path
    }

    /// Resolve a module based on a path.
    #[allow(clippy::needless_pass_by_value)]
    fn impl_resolve(
        &self,
        global: Option<&mut rhai::GlobalRuntimeState>,
        source: Option<&str>,
        path: &str,
        position: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        // Load relative paths from source if there is no base path specified
        let source_path = global
            .as_ref()
            .and_then(|g| g.source())
            .or(source)
            .and_then(|p| std::path::Path::new(p).parent());

        let path = self.get_file_path(path, source_path);

        if !path.exists() {
            return Err(Box::new(rhai::EvalAltResult::ErrorModuleNotFound(
                path.to_str()
                    .map_or_else(String::default, std::string::ToString::to_string),
                position,
            )));
        }

        if self.is_cache_enabled() {
            let module = { locked_read(&self.cache).get(&path).cloned() };

            if let Some(module) = module {
                Ok(module)
            } else {
                let module = locked_write(&self.loader).load(path.as_path())?;
                locked_write(&self.cache).insert(path, module.clone());

                Ok(module)
            }
        } else {
            locked_write(&self.loader).load(path.as_path())
        }
    }
}

impl rhai::ModuleResolver for DylibModuleResolver {
    fn resolve(
        &self,
        _: &rhai::Engine,
        source: Option<&str>,
        path: &str,
        position: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        self.impl_resolve(None, source, path, position)
    }

    fn resolve_raw(
        &self,
        _: &rhai::Engine,
        global: &mut rhai::GlobalRuntimeState,
        _: &mut rhai::Scope,
        path: &str,
        position: rhai::Position,
    ) -> Result<rhai::Shared<rhai::Module>, Box<rhai::EvalAltResult>> {
        self.impl_resolve(Some(global), None, path, position)
    }

    /// This resolver is Rust based, so it cannot resolve ASTs.
    /// This function will always return `None`.
    fn resolve_ast(
        &self,
        _: &rhai::Engine,
        _: Option<&str>,
        _: &str,
        _: rhai::Position,
    ) -> Option<Result<rhai::AST, Box<rhai::EvalAltResult>>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhai::ModuleResolver;

    fn build_test_plugin() -> &'static std::path::PathBuf {
        // Prevents multiple threads writing to the dll on windows and triggering a STATUS_ACCESS_VIOLATION error.
        static PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
        PATH.get_or_init(|| {
            let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let status = std::process::Command::new("cargo")
                .args(["build", "--example", "test_plugin"])
                .current_dir(manifest_dir)
                .status()
                .expect("failed to execute cargo build");

            assert!(status.success(), "building test_plugin failed");

            let target_dir = std::env::var("CARGO_TARGET_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| manifest_dir.join("target"));

            #[cfg(target_os = "linux")]
            return target_dir.join("debug/examples/libtest_plugin.so");
            #[cfg(target_os = "macos")]
            return target_dir.join("debug/examples/libtest_plugin.dylib");
            #[cfg(target_os = "windows")]
            return target_dir.join("debug/examples/test_plugin.dll");
        })
    }

    fn test_plugin_module_path() -> String {
        build_test_plugin()
            .with_extension("")
            .to_str()
            .unwrap()
            .replace('\\', "/")
    }

    #[test]
    fn new() {
        let mut r = DylibModuleResolver::new();
        let rp = DylibModuleResolver::with_path("./scripts");

        r.enable_cache(false);
        assert!(!r.is_cache_enabled());
        assert!(rp.is_cache_enabled());
    }

    #[test]
    fn file_path_resolution() {
        let r = DylibModuleResolver::new();

        let relative = r.get_file_path("mylib", None);

        #[cfg(target_os = "linux")]
        assert_eq!(relative, std::path::PathBuf::from("mylib.so"));
        #[cfg(target_os = "windows")]
        assert_eq!(relative, std::path::PathBuf::from("mylib.dll"));
        #[cfg(target_os = "macos")]
        assert_eq!(relative, std::path::PathBuf::from("mylib.dylib"));

        let source = r.get_file_path("mylib", Some(std::path::Path::new("source")));

        #[cfg(target_os = "linux")]
        assert_eq!(source, std::path::PathBuf::from("source/mylib.so"));
        #[cfg(target_os = "windows")]
        assert_eq!(source, std::path::PathBuf::from("source/mylib.dll"));
        #[cfg(target_os = "macos")]
        assert_eq!(source, std::path::PathBuf::from("source/mylib.dylib"));
    }

    #[test]
    fn file_path_resolution_with_path() {
        let rp = DylibModuleResolver::with_path("scripts");

        let relative = rp.get_file_path("mylib", None);
        #[cfg(target_os = "linux")]
        assert_eq!(relative, std::path::PathBuf::from("scripts/mylib.so"));
        #[cfg(target_os = "windows")]
        assert_eq!(relative, std::path::PathBuf::from("scripts/mylib.dll"));
        #[cfg(target_os = "macos")]
        assert_eq!(relative, std::path::PathBuf::from("scripts/mylib.dylib"));

        // TODO: add tests for all platforms.
        let absolute = rp.get_file_path("/usr/local/lib/mylib", None);
        #[cfg(target_os = "linux")]
        assert_eq!(
            absolute,
            std::path::PathBuf::from("/usr/local/lib/mylib.so")
        );
    }

    #[test]
    fn resolve_ast_returns_none() {
        let r = DylibModuleResolver::new();
        let engine = rhai::Engine::new();
        assert!(r
            .resolve_ast(&engine, None, "anything", rhai::Position::NONE)
            .is_none());
    }

    #[test]
    fn resolve_returns_error_for_missing_file() {
        let r = DylibModuleResolver::new();
        let engine = rhai::Engine::new();
        assert!(r
            .resolve(&engine, None, "nonexistent_module", rhai::Position::NONE)
            .is_err());
    }

    #[test]
    fn resolve_loads_module() {
        let module_path = test_plugin_module_path();
        let r = DylibModuleResolver::new();
        let engine = rhai::Engine::new();
        let module = r
            .resolve(&engine, None, &module_path, rhai::Position::NONE)
            .expect("failed to resolve module");
        assert!(!module.is_empty());
    }

    #[test]
    fn resolve_cache_hit_returns_same_module() {
        let module_path = test_plugin_module_path();
        let r = DylibModuleResolver::new();
        let engine = rhai::Engine::new();
        let m1 = r
            .resolve(&engine, None, &module_path, rhai::Position::NONE)
            .expect("first resolve failed");
        let m2 = r
            .resolve(&engine, None, &module_path, rhai::Position::NONE)
            .expect("second resolve failed");

        assert!(std::ptr::eq(&*m1, &*m2));
    }

    #[test]
    fn resolve_without_cache() {
        let module_path = test_plugin_module_path();
        let mut r = DylibModuleResolver::new();

        r.enable_cache(false);

        let engine = rhai::Engine::new();

        r.resolve(&engine, None, &module_path, rhai::Position::NONE)
            .expect("resolve without cache failed");
    }

    #[test]
    fn resolve_raw_via_engine_import() {
        let module_path = test_plugin_module_path();
        let _ = rhai::config::hashing::set_hashing_seed(Some([1, 2, 3, 4]));
        let mut engine = rhai::Engine::new();

        engine.set_module_resolver(DylibModuleResolver::new());

        let result = engine
            .eval::<rhai::INT>(&format!(r#"import "{module_path}" as p; p::add(1, 2)"#))
            .expect("engine eval failed");

        assert_eq!(result, 3);
    }
}
