use brotli::enc::{BrotliCompress, BrotliEncoderParams};
use rayon::prelude::*;
use std::{error::Error, fs, path::Path};
use strum::VariantArray;
use tutorlolv2::{CastId, ChampionId, ItemId, RuneId, ValueId, yew::render};

type MayFail<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

static CSS: &str = include_str!("../style.css");
static PAGE_CSS: &str = "h1,h2{color:#d4d4d4;font-family:sans-serif}";

/// Generates `html/{champions,items,runes}/{Variant}.html.br` at the workspace
/// root. The server reads these files from its static folder at runtime, so the
/// whole `html` folder has to be uploaded next to `img`.
fn main() -> MayFail {
    println!("cargo:rerun-if-changed=build.rs");

    let html_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Server crate must live inside the workspace")?
        .join("html");

    write_pages(&html_dir.join("champions"), ChampionId::VARIANTS, |id| {
        let functions = id
            .abilities()
            .iter()
            .map(|metadata| id.render_fn(metadata.kind))
            .collect::<render::MayFail<Vec<_>>>()?;

        Ok(functions.join("\n"))
    })?;

    write_pages(
        &html_dir.join("items"),
        ItemId::VARIANTS,
        ValueId::render_fn,
    )?;

    write_pages(
        &html_dir.join("runes"),
        RuneId::VARIANTS,
        ValueId::render_fn,
    )?;

    Ok(())
}

fn write_pages<T, F>(dir: &Path, ids: &[T], functions: F) -> MayFail
where
    T: CastId + Sync,
    F: Fn(&T) -> render::MayFail<String> + Sync,
{
    // Recreate the folder so pages of removed entities do not survive a new patch
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }

    fs::create_dir_all(dir)?;

    ids.par_iter().try_for_each(|id| -> MayFail {
        let page = render_page(id, &functions)
            .map_err(|e| format!("Rendering docs of {}: {e}", id.debug()))?;

        let params = BrotliEncoderParams {
            quality: 11,
            lgwin: 22,
            ..Default::default()
        };

        let mut compressed = Vec::new();

        BrotliCompress(&mut page.as_bytes(), &mut compressed, &params)?;

        fs::write(dir.join(id.debug()).with_extension("html.br"), compressed)?;

        Ok(())
    })
}

fn render_page<T: CastId>(
    id: &T,
    functions: impl Fn(&T) -> render::MayFail<String>,
) -> render::MayFail<String> {
    let title = id.name().replace('&', "&amp;").replace('<', "&lt;");
    let definition = section("Definition", &id.render_global()?);
    let formulas = section("Formulas", &functions(id)?);
    let generator = id
        .render_generator()
        .map(|code| section("Generator", &code))
        .unwrap_or_default();

    Ok(format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>{title}</title><style>{CSS}{PAGE_CSS}</style></head><body><h1>{title}</h1>{definition}{formulas}{generator}</body></html>"#
    ))
}

fn section(title: &str, code: &str) -> String {
    format!("<h2>{title}</h2><pre>{code}</pre>")
}
