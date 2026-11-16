use std::{env, error::Error, io, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-check-cfg=cfg(wrong_target)");

    // Rust-analyzer runs every build script for the host target, so a panic here
    // would fail its build scripts step. The crate reports it instead, see main.rs
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        println!("cargo:rustc-cfg=wrong_target");
        return Ok(());
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WSL_DISTRO_NAME");

    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("set by Cargo"));
    let workspace = manifest
        .parent()
        .and_then(|parent| parent.parent())
        .expect("native lives under apps/native");

    let images = workspace.join("img").join("raw");

    // Build scripts run on the host. A Linux path must be translated before it
    // is embedded into a Windows executable; wslpath also handles /mnt/c drives.
    let windows_path = if cfg!(windows) {
        images.to_str().ok_or("Image path is not UTF-8")?.to_owned()
    } else {
        let output = Command::new("wslpath")
            .arg("-w")
            .arg(&images)
            .output()
            .map_err(|error| {
                io::Error::other(format!("Mapping image path with wslpath: {error}"))
            })?;

        if !output.status.success() {
            return Err(io::Error::other(format!(
                "Cannot map image path {images:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
            .into());
        }

        let path = String::from_utf8(output.stdout)?;
        let path = path.trim_end_matches(['\r', '\n']);

        if path.is_empty() {
            return Err(io::Error::other("wslpath returned an empty image path").into());
        }
        path.to_owned()
    };

    println!("cargo:rustc-env=TUTORLOLV2_DEFAULT_RAW_IMG={windows_path}");

    // gpui calls TaskDialogIndirect, which only exists in comctl32 v6. Windows
    // loads v6 only when the executable's manifest asks for it.
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTDEPENDENCY:type='win32' \
         name='Microsoft.Windows.Common-Controls' version='6.0.0.0' \
         processorArchitecture='*' publicKeyToken='6595b64144ccf1df' language='*'"
    );

    Ok(())
}
