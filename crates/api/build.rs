//! Keeps `src/generated.rs` in sync with the server's `api_endpoint!` declarations.
//!
//! The file stays committed so contract changes are reviewed in diffs, and CI still
//! checks it with `cargo run -p api_codegen -- check`. Builds outside this
//! workspace (no server sources next to this crate) use the committed file as is.

use api_codegen::{GENERATED_FILE, ROUTES_DIR, write_if_changed};
use std::path::Path;

fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("set by cargo");
    let root = Path::new(&manifest)
        .ancestors()
        .nth(2)
        .expect("crate lives in the workspace");

    let (routes, output) = (root.join(ROUTES_DIR), root.join(GENERATED_FILE));

    println!("cargo::rerun-if-changed=build.rs");

    if !routes.is_dir() {
        return;
    }

    // A directory is rescanned for any modified file inside it.
    println!("cargo::rerun-if-changed={}", routes.display());
    println!("cargo::rerun-if-changed={}", output.display());

    if let Err(error) = write_if_changed(&routes, &output) {
        panic!(
            "generating endpoint contracts from {}: {error}",
            routes.display()
        );
    }
}
