use std::{fs, process::Command};
use tutorlolv2_build_dep::scripts::spells::spells_code;
use tutorlolv2_wiki::spells::SpellData;

#[test]
fn generated_ignite_uses_changed_wiki_data_in_const_contexts() {
    let data = SpellData::from_lua(r#"return { Ignite = { effect = "Deals {{as|{{pp|100;then +30*x|1 to 20 by 1}} {{sti|true damage}}}}." } }"#).unwrap();
    let code = spells_code(&data).unwrap();
    let dir = std::env::temp_dir().join(format!("ignite-codegen-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("main.rs");
    let binary = dir.join("ignite");
    fs::write(&source, format!("{code}\nconst FIRST: i32 = ignite(core::num::NonZeroU8::new(1).unwrap());\nfn main() {{ assert_eq!(FIRST, 100); for (level, want) in [(5,220), (20,670), (21,700), (255,7720)] {{ assert_eq!(ignite(core::num::NonZeroU8::new(level).unwrap()), want); }} }}")).unwrap();
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(Command::new(&binary).status().unwrap().success());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn invalid_parsed_cache_is_rejected_before_generating_code() {
    for ignite in [vec![], vec![10], vec![10, -20], vec![0, i32::MAX]] {
        assert!(spells_code(&SpellData { ignite }).is_err());
    }
}
