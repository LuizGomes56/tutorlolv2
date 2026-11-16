fn main() {
    println!("cargo:rustc-check-cfg=cfg(wrong_target)");

    // Rust-analyzer runs every build script for the host target, so a panic here
    // would fail its build scripts step. The crate reports it instead, see main.rs
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") {
        println!("cargo:rustc-cfg=wrong_target");
    }
}
