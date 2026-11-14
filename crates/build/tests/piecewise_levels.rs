use build::scripts::utils::cast_f32;
use std::{fs, process::Command};
use types::CtxVar;
use wiki::wikitext::Formula;

#[test]
fn emitted_three_segment_match_compiles_and_preserves_every_level() {
    let formula = Formula::Table {
        var: CtxVar::Level,
        keys: (1..=18).collect(),
        values: vec![
            35., 38., 41., 44., 47., 50., 53., 62., 71., 80., 89., 98., 107., 122., 137., 152.,
            167., 182.,
        ],
    };
    let body = cast_f32(&formula.rust()).unwrap();
    let expr = syn::parse_str::<syn::Expr>(&body).unwrap();
    let syn::Expr::Match(expr) = expr else {
        panic!("Expected grouped match")
    };
    assert_eq!(expr.arms.len(), 3);
    let source = format!(
        r#"
struct Ctx {{level:f32}}
const fn damage(ctx:&Ctx)->f32 {{ {body} }}
const HIGH:f32=damage(&Ctx {{level:255.0}});
fn main() {{
 assert_eq!(HIGH,3737.0);
 for level in 0..=255 {{
   let n=level as f32;
   let expected=if level<8 {{35.0+3.0*(n-1.0).max(0.0)}} else if level<14 {{62.0+9.0*(n-8.0)}} else {{122.0+15.0*(n-14.0)}};
   assert_eq!(damage(&Ctx {{level:n}}),expected);
 }}
}}
"#
    );
    let dir = std::env::temp_dir().join(format!("piecewise-codegen-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.rs");
    let binary = dir.join("run");
    fs::write(&path, source).unwrap();
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&path)
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
