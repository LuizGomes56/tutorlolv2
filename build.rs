#[path = "build/mod.rs"]
mod build;

use build::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{env, fs, path::Path, time::SystemTime};

struct BuildArgs {
    rerun_if_changed: &'static [&'static str],
    generated_files: &'static [&'static str],
    source_file: &'static str,
    function_name: Box<dyn Fn(&str) + Send + Sync>,
}

fn main() {
    if std::env::var_os("SKIP_CODEGEN").is_some() {
        eprintln!("Codegen ignored");
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let maybe_run = vec![
        BuildArgs {
            rerun_if_changed: &["build/generator_runner.rs"],
            generated_files: &["generator_runner.rs"],
            source_file: "build/generator_runner.rs",
            function_name: Box::new(generator_runner),
        },
        BuildArgs {
            rerun_if_changed: &["build/meta_items.rs", "internal/meta_items.json"],
            generated_files: &["internal_meta.rs"],
            source_file: "build/meta_items.rs",
            function_name: Box::new(internal_meta_items),
        },
        BuildArgs {
            rerun_if_changed: &["build/export_code.rs"],
            generated_files: &["export_code.br", "comptime_exports/export_code.br"],
            source_file: "build/export_code.rs",
            function_name: Box::new(export_code),
        },
        BuildArgs {
            rerun_if_changed: &["build/sprite_map.rs", "sprite/sprite.json"],
            generated_files: &["sprite_map.br"],
            source_file: "build/sprite_map.rs",
            function_name: Box::new(generate_sprite_map),
        },
    ];

    println!("cargo:rerun-if-changed=build.rs");
    for build_args in &maybe_run {
        for &path in build_args.rerun_if_changed {
            println!("cargo:rerun-if-changed={}", path);
        }
    }

    let tasks_to_run: Vec<_> = maybe_run
        .iter()
        .filter(|build_args| needs_rebuild(build_args, Path::new(&out_dir)))
        .collect();

    if tasks_to_run.is_empty() {
        println!("All generated files are up to date - skipping all generators");
        return;
    }

    println!("Running {} generator(s) in parallel...", tasks_to_run.len());

    tasks_to_run.par_iter().for_each(|build_args| {
        execute_build_task(build_args, &out_dir);
    });

    println!("All generators completed successfully!");
}

fn get_newest_modification_time<P: AsRef<Path>>(paths: &[P]) -> Option<SystemTime> {
    paths
        .iter()
        .filter_map(|path| {
            let path_ref = path.as_ref();

            if path_ref.is_dir() {
                get_newest_in_directory(path_ref)
            } else {
                fs::metadata(path_ref).and_then(|meta| meta.modified()).ok()
            }
        })
        .max()
}

fn get_newest_in_directory(dir: &Path) -> Option<SystemTime> {
    if let Ok(entries) = fs::read_dir(dir) {
        entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    get_newest_in_directory(&path)
                } else {
                    fs::metadata(&path).and_then(|meta| meta.modified()).ok()
                }
            })
            .max()
    } else {
        None
    }
}

fn get_oldest_generated_time(out_dir: &Path, generated_files: &[&str]) -> Option<SystemTime> {
    generated_files
        .iter()
        .filter_map(|file| {
            let file_path = out_dir.join(file);
            fs::metadata(&file_path)
                .and_then(|meta| meta.modified())
                .ok()
        })
        .min()
}

fn needs_rebuild(build_args: &BuildArgs, out_dir: &Path) -> bool {
    let newest_input_time = match get_newest_modification_time(build_args.rerun_if_changed) {
        Some(time) => time,
        None => {
            println!(
                "cargo:warning=Could not get modification time for {}",
                build_args.source_file
            );
            return true;
        }
    };

    let oldest_output_time = match get_oldest_generated_time(out_dir, build_args.generated_files) {
        Some(time) => time,
        None => return true,
    };

    oldest_output_time < newest_input_time
}

fn execute_build_task(build_args: &BuildArgs, out_dir: &str) {
    let out_path = Path::new(out_dir);

    if needs_rebuild(build_args, out_path) {
        println!("Generating from {}...", build_args.source_file);
        (build_args.function_name)(out_dir);

        println!(
            "Generated {} files successfully",
            build_args.generated_files.len()
        );
    } else {
        println!(
            "Skipping {}; all outputs are up to date",
            build_args.source_file
        );
    }
}
