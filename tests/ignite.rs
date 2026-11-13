use core::num::NonZeroU8;
use tutorlolv2::ignite;

#[test]
fn ignite_uses_the_wiki_level_progression_and_is_const_callable() {
    const FIRST: i32 = ignite(NonZeroU8::new(1).unwrap());
    assert_eq!(FIRST, 70);
    for (level, expected) in [
        (4, 130),
        (5, 150),
        (6, 175),
        (18, 475),
        (20, 525),
        (21, 550),
        (255, 6400),
    ] {
        assert_eq!(
            ignite(NonZeroU8::new(level).unwrap()),
            expected,
            "level {level}"
        );
    }
}

#[cfg(feature = "formula-docs")]
#[test]
fn displayed_ignite_evaluates_to_the_generated_damage_at_every_level() {
    use std::{fs, process::Command};
    let displayed =
        tutorlolv2::yew::render::FormulaRenderer::ignite_fn_markup::<tutorlolv2_codec::Spans>()
            .text;
    let expected = (1..=255)
        .map(|level| (level, ignite(NonZeroU8::new(level).unwrap())))
        .collect::<Vec<_>>();
    let dir = std::env::temp_dir().join(format!("ignite-display-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("main.rs");
    let binary = dir.join("ignite");
    fs::write(&source, format!("use core::num::NonZeroU8;\n{displayed}\nfn main() {{ for (level, expected) in {expected:?} {{ assert_eq!(ignite(NonZeroU8::new(level).unwrap()), expected); }} }}")).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert!(Command::new(binary).status().unwrap().success());
    fs::remove_dir_all(dir).unwrap();
}
