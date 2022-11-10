use engine::build_engine;

#[cfg(not(target_os = "linux"))]
#[cfg(not(target_os = "windows"))]
compile_error!("unsupported platform - only Linux & Windows are supported");

fn main() {
    let engine = build_engine();

    engine
        .run(
            r#"
print_stuff();
debug(triple_add(1, 2, 3));
debug(get_property(#{ property: "value" }));

let object = new_plugin_object("stuff to display");
object.display_inner();
"#,
        )
        .expect("failed to execute script");
}
