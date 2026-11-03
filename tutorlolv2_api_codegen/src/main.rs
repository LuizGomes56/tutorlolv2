use std::{
    env, fs,
    path::{Path, PathBuf},
};
fn collect(path: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = env::args().nth(1).unwrap_or_else(|| "check".into());
    if !["generate", "check"].contains(&mode.as_str()) {
        return Err("usage: tutorlolv2_api_codegen [generate|check]".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let routes = root.join("tutorlolv2_server/src/server/routes");
    let mut files = vec![];
    collect(&routes, &mut files)?;
    files.sort();
    let sources = files
        .iter()
        .map(|p| {
            Ok((
                p.strip_prefix(&routes)?.to_string_lossy().into_owned(),
                fs::read_to_string(p)?,
            ))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let refs = sources
        .iter()
        .map(|(name, source)| (name.as_str(), source.as_str()))
        .collect::<Vec<_>>();
    let generated = tutorlolv2_api_codegen::generate_sources(&refs)?;
    let output = root.join("tutorlolv2_api/src/generated.rs");
    if mode == "generate" {
        fs::write(output, generated)?;
    } else if fs::read_to_string(output)? != generated {
        return Err(
            "endpoint contracts are stale; run cargo run -p tutorlolv2_api_codegen -- generate"
                .into(),
        );
    }
    Ok(())
}
