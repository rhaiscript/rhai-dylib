use rhai_dylib::{module_resolvers::dylib::DylibModuleResolver, rhai};

fn main() {
    let mut engine = rhai::Engine::new();

    engine.set_module_resolver(DylibModuleResolver::new());

    engine
        .run(
            r#"
import "../simple/plugin/target/debug/libplugin" as plugin;

print_stuff();
debug(triple_add(1, 2, 3));
    
    "#,
        )
        .unwrap();
}
