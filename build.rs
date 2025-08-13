#[path = "build/mod.rs"]
mod build;

use build::*;

#[tokio::main]
async fn main() {
    if std::env::var_os("GENERATE").is_none() {
        eprintln!("GENERATE is not set, codegen will be ignored");
        return;
    }

    export_code().await;
}
