use std::{env, process::Command};

fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("rustc --version");

    println!(
        "cargo:rustc-env=PERF_RUSTC={}",
        String::from_utf8_lossy(&version.stdout).trim()
    );

    for (source, destination) in [
        ("TARGET", "PERF_TARGET"),
        ("OPT_LEVEL", "PERF_OPT_LEVEL"),
        ("CARGO_ENCODED_RUSTFLAGS", "PERF_RUSTFLAGS"),
    ] {
        println!("cargo:rerun-if-env-changed={source}");
        println!(
            "cargo:rustc-env={destination}={}",
            env::var(source).unwrap_or_default().replace('\x1f', " ")
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
}
