/// A plugin trait implementation.
pub mod plugin;
/// Trait implementation to create objects that load plugins.
pub mod plugin_loader;

/// Re-exporting rhai to prevent version mismatch.
pub use rhai;
