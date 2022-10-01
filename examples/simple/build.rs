fn main() {
    eprintln!(
        "{}",
        format!("{}/plugin/Cargo.toml", env!("CARGO_MANIFEST_DIR"))
    );

    std::process::Command::new("cargo")
        .args(&[
            "build",
            "--manifest-path",
            &format!("{}/plugin/Cargo.toml", env!("CARGO_MANIFEST_DIR")),
        ])
        .status()
        .unwrap();
}
