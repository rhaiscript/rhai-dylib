use plugin_trait::Loader;

fn main() {
    println!("main: {:?}", std::any::TypeId::of::<rhai::Map>());

    let loader = Loader::default();

    let mut engine = rhai::Engine::new();

    // Register the plugin's module into the engine.
    loader
        .plugins
        .iter()
        .for_each(|plugin| plugin.register(plugin_trait::Builder::new(&mut engine)));

    // checking if the module has been registered.
    println!("{:#?}", engine.gen_fn_signatures(false));

    engine.run("with_params(#{});").unwrap();
}
