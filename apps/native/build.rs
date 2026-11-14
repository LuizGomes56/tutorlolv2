use std::{env, error::Error, io, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        panic!("native requires --target x86_64-pc-windows-msvc");
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

    Ok(())
}
