#[path = "build/mod.rs"]
mod build;

use build::{
    phf_champions::global_phf_internal_champions, phf_items::global_phf_internal_items,
    phf_meta::global_phf_internal_meta_items, phf_names::global_phf_internal_names,
    phf_runes::global_phf_internal_runes, writer_match_arms::writer_match_arms,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::build::phf_formulas::global_phf_formulas;

fn maybe_run<F>(src: &str, out_file: &Path, gen_fn: F)
where
    F: Fn(&str),
{
    let src_meta = fs::metadata(src).unwrap_or_else(|_| panic!("file/folder not found: '{}'", src));
    let src_mod = src_meta
        .modified()
        .unwrap_or_else(|_| panic!("Error on modified(): '{}'", src));

    let needs = match fs::metadata(out_file) {
        Ok(out_meta) => {
            let out_mod = out_meta
                .modified()
                .unwrap_or_else(|_| panic!("Error on modified(): '{:?}'", out_file));
            out_mod < src_mod
        }
        Err(_) => true,
    };

    if needs {
        println!("Generating from: '{}'...", src);
        let out_dir = out_file
            .parent()
            .expect("out_file should have parent")
            .to_str()
            .unwrap();
        gen_fn(out_dir);
    } else {
        println!("Skipping '{}'; artifact already updated.", src);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/writers");
    println!("cargo:rerun-if-changed=build/phf_champions.rs");
    println!("cargo:rerun-if-changed=build/phf_phf_items.rs");
    println!("cargo:rerun-if-changed=build/phf_runes.rs");
    println!("cargo:rerun-if-changed=build/phf_meta.rs");
    println!("cargo:rerun-if-changed=build/phf_names.rs");
    println!("cargo:rerun-if-changed=build/phf_formulas.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out = PathBuf::from(&out_dir);

    maybe_run("src/writers", &out.join("writers_generated.rs"), |o| {
        writer_match_arms(o)
    });
    maybe_run(
        "build/phf_champions.rs",
        &out.join("internal_champions.rs"),
        |o| global_phf_internal_champions(o),
    );
    maybe_run("build/phf_items.rs", &out.join("internal_items.rs"), |o| {
        global_phf_internal_items(o)
    });
    maybe_run("build/phf_runes.rs", &out.join("internal_runes.rs"), |o| {
        global_phf_internal_runes(o)
    });
    maybe_run("build/phf_meta.rs", &out.join("internal_meta.rs"), |o| {
        global_phf_internal_meta_items(o)
    });
    maybe_run("build/phf_names.rs", &out.join("internal_names.rs"), |o| {
        global_phf_internal_names(o)
    });
    maybe_run(
        "build/phf_formulas.rs",
        &out.join("const_formulas.rs"),
        |o| global_phf_formulas(o),
    );
}
