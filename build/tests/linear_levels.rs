use std::{fs, process::Command};
use tutorlolv2_build_dep::scripts::utils::cast_f32;
use tutorlolv2_types::CtxVar;
use tutorlolv2_wiki::wikitext::Formula;

#[test]
fn emitted_affine_growth_compiles_and_evaluates_through_u8_maximum() {
    let rank = Formula::ByRank {
        rank: CtxVar::QLevel,
        values: vec![10.0, 20.0, 30.0, 40.0],
    };
    let level = Formula::Table {
        var: CtxVar::Level,
        keys: vec![1, 2, 3, 4],
        values: vec![10.0, 20.0, 30.0, 40.0],
    };
    let shifted = Formula::Table {
        var: CtxVar::Level,
        keys: vec![6, 7, 8],
        values: vec![100.0, 120.0, 140.0],
    };
    let nonlinear = Formula::ByRank {
        rank: CtxVar::RLevel,
        values: vec![375.0, 630.0, 945.0],
    };
    let numeric = Formula::NumericTable {
        axis: Box::new(Formula::Var(CtxVar::Level)),
        keys: vec![1.0, 2.0, 3.0],
        values: vec![10.0, 20.0, 30.0],
        interpolated: false,
    };
    let decreasing = Formula::Table {
        var: CtxVar::Level,
        keys: vec![1, 2, 3],
        values: vec![100.0, 90.0, 80.0],
    };
    let mut functions = String::new();
    for (name, formula) in [
        ("rank", rank),
        ("level", level),
        ("shifted", shifted),
        ("nonlinear", nonlinear),
        ("numeric", numeric),
        ("decreasing", decreasing),
    ] {
        let body = cast_f32(&formula.rust()).unwrap();
        if name != "nonlinear" {
            assert!(!body.contains("match"), "{body}");
        }
        functions.push_str(&format!("const fn {name}(ctx:&Ctx)->f32 {{ {body} }}\n"));
    }
    let source = format!(
        r#"
struct Levels {{r:u8}}
struct Ctx {{level:f32,rank:u8,ability_levels:Levels}}
impl Ctx {{const fn q_level(&self)->f32 {{self.rank as f32}} const fn r_level(&self)->f32 {{self.rank as f32}}}}
{functions}
const HIGH: f32=rank(&Ctx {{level:255.0,rank:255,ability_levels:Levels{{r:255}}}});
fn main() {{
 assert_eq!(HIGH,2550.0);
 for n in [0u8,1,5,6,18,19,255] {{
   let ctx=Ctx {{level:n as f32,rank:n,ability_levels:Levels{{r:n}}}};
   let expected=10.0*(n as f32).max(1.0);
   assert_eq!(rank(&ctx),expected);
   assert_eq!(level(&ctx),expected);
   assert_eq!(numeric(&ctx),expected);
   assert_eq!(decreasing(&ctx),100.0-10.0*(n as f32-1.0).max(0.0));
   assert_eq!(shifted(&ctx),100.0+20.0*(n as f32-6.0).max(0.0));
 }}
 assert_eq!(nonlinear(&Ctx {{level:255.0,rank:255,ability_levels:Levels{{r:255}}}}),945.0);
}}
"#
    );
    let dir = std::env::temp_dir().join(format!("linear-levels-codegen-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.rs");
    let binary = dir.join("linear-levels");
    fs::write(&path, source).unwrap();
    let output = Command::new("rustc")
        .args(["--edition=2024"])
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
