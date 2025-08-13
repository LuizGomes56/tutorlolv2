#[path = "build/mod.rs"]
mod build;

use build::*;
use std::env;

#[tokio::main]
async fn main() {
    if std::env::var_os("GENERATE").is_none() {
        eprintln!("GENERATE is not set, codegen will be ignored");
        return;
    }

    export_code().await;

    let out_dir = env::var("OUT_DIR").unwrap();
    internal_meta_items(&out_dir);
}
