use std::{env, fs, path::Path};
use tutorlolv2_api_codegen::{GENERATED_FILE, ROUTES_DIR, generate_from_routes, write_if_changed};

fn main() -> Result<(), String> {
    let mode = env::args().nth(1).unwrap_or_else(|| "check".into());
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let (routes, output) = (root.join(ROUTES_DIR), root.join(GENERATED_FILE));

    match mode.as_str() {
        "generate" => write_if_changed(&routes, &output).map(|_| ()),
        "check" => {
            let current = fs::read_to_string(&output).map_err(|e| e.to_string())?;

            if current == generate_from_routes(&routes)? {
                Ok(())
            } else {
                Err("endpoint contracts are stale; build tutorlolv2_api or run \
                     `cargo run -p tutorlolv2_api_codegen -- generate`"
                    .into())
            }
        }
        _ => Err("usage: tutorlolv2_api_codegen [generate|check]".into()),
    }
}
