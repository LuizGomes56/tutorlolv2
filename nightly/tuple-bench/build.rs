//! Benchmark-only generation; does not change the library or its generator.
use std::{collections::BTreeMap, fmt::Write, path::PathBuf};
use syn::{Expr, Item, Member};
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let ctx = tutorlolv2_nightly_build::BuildContext::new(
        tutorlolv2_nightly_build::project_root(),
        &out,
        false,
    )?;
    ctx.run()?;
    let source = std::fs::read_to_string(out.join("callables.rs"))?;
    let re = regex::Regex::new(r"Self::(\w+) => ([\w:]+)\(")?;
    let paths = re
        .captures_iter(&source)
        .map(|c| (c[1].to_owned(), c[2].replace("crate::", "new::")))
        .collect::<BTreeMap<_, _>>();
    let mut generated = String::new();
    for (group, id_ty) in [
        ("champions", "ChampionId"),
        ("items", "ItemId"),
        ("runes", "RuneId"),
    ] {
        let mut rows = BTreeMap::new();
        for entry in std::fs::read_dir(out.join(group))? {
            let file = entry?.path();
            if file.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let variant = file.file_stem().unwrap().to_str().unwrap().to_owned();
            let parsed = syn::parse_file(&std::fs::read_to_string(file)?)?;
            let mut fields = BTreeMap::new();
            for item in parsed.items {
                let Item::Static(item) = item else { continue };
                let Expr::Struct(value) = *item.expr else {
                    continue;
                };
                for field in value.fields {
                    let Member::Named(name) = field.member else {
                        continue;
                    };
                    let name = name.to_string();
                    if !matches!(
                        name.as_str(),
                        "closures" | "ability_batch" | "melee" | "ranged"
                    ) {
                        continue;
                    }
                    let expr = match field.expr {
                        Expr::Reference(r) => *r.expr,
                        e => e,
                    };
                    let entries = match expr {
                        Expr::Array(a) => a.elems.into_iter().collect::<Vec<_>>(),
                        e => vec![e],
                    };
                    let mut fs = vec![];
                    for e in entries {
                        let Expr::Path(p) = e else {
                            panic!("expected callable")
                        };
                        fs.push(paths[&p.path.segments.last().unwrap().ident.to_string()].clone());
                    }
                    fields.insert(name, fs);
                }
            }
            rows.insert(variant, fields);
        }
        let fields: &[&str] = if group == "champions" {
            &["closures"]
        } else {
            &["melee", "ranged"]
        };
        for &field in fields {
            let label = format!("{group}_{field}");
            writeln!(
                generated,
                "pub const fn pointers_{label}(id: new::{id_ty}) -> &'static [Formula] {{ match id {{"
            )?;
            for (variant, fs) in &rows {
                writeln!(
                    generated,
                    "new::{id_ty}::{variant} => &[{}],",
                    fs[field].join(",")
                )?;
            }
            writeln!(generated, "}} }}")?;
            let max = rows.values().map(|r| r[field].len()).max().unwrap();
            assert!(max <= 18, "increase tuple arity: {group}/{field} has {max}");
            for (operation, helper, result, extra, args) in [
                ("raw", "raw", "()", "", "ctx, &group, out"),
                (
                    "eval",
                    if group == "champions" {
                        "abilities"
                    } else {
                        "pairs"
                    },
                    "()",
                    ", onhit: &mut RangeDamage, modifiers: Modifiers",
                    if group == "champions" {
                        "ctx, onhit, modifiers, id.metadata(), &group, out"
                    } else {
                        "ctx, onhit, modifiers, &[id.metadata()], &group, out"
                    },
                ),
            ] {
                if operation == "raw" && group != "champions" {
                    continue;
                }
                writeln!(
                    generated,
                    "pub const fn tuple_{operation}_{label}(id: new::{id_ty}, ctx: &Ctx, out: &mut [{}]{extra}) -> {result} {{ match id {{",
                    if operation == "raw" { "f32" } else { "i32" }
                )?;
                for (variant, fs) in &rows {
                    writeln!(
                        generated,
                        "new::{id_ty}::{variant} => {{ let group=({},); {helper}({args}); }},",
                        fs[field].join(",")
                    )?;
                }
                writeln!(generated, "}} }}")?;
            }
        }
        if group == "champions" {
            writeln!(
                generated,
                "pub fn tuple_calculator(game:new::model::InputGame<'_>) -> crate::engine::Output {{ match game.active_player.data.champion_id {{"
            )?;
            for (variant, fs) in &rows {
                writeln!(
                    generated,
                    "new::ChampionId::{variant} => crate::engine::calculator(game, &crate::engine::Tuples {{ group: ({},) }}),",
                    fs["closures"].join(",")
                )?;
            }
            writeln!(generated, "}} }}")?;
            writeln!(
                generated,
                "pub const fn pointer_batch(id: new::ChampionId) -> Batch {{ match id {{"
            )?;
            for (variant, fs) in &rows {
                writeln!(
                    generated,
                    "new::ChampionId::{variant} => {},",
                    fs["ability_batch"][0]
                )?;
            }
            writeln!(generated, "}} }}")?;
        }
    }
    std::fs::write(out.join("tuple_groups.rs"), generated)?;
    Ok(())
}
