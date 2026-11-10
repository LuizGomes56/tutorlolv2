//! Replace erased pointers in generated tables with homogeneous const callables.
use crate::{BuildContext, MayFail};
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::{collections::BTreeMap, fmt::Write};
use syn::{Expr, Item, Member, spanned::Spanned};

impl BuildContext {
    pub fn generate_callables(&self) -> MayFail {
        let mut formulas = BTreeMap::<String, String>::new();
        let mut batches = BTreeMap::<String, String>::new();
        formulas.insert("Zero".into(), "crate::libgen::zero".into());
        for group in ["champions", "items", "runes"] {
            for entry in std::fs::read_dir(self.out.join(group))? {
                let path = entry?.path();
                if path.extension().is_none_or(|ext| ext != "rs") {
                    continue;
                }
                let module = path.file_stem().unwrap().to_str().unwrap().to_snake_case();
                let source = std::fs::read_to_string(&path)?;
                let parsed = syn::parse_file(&source)?;
                let mut replacements = Vec::new();
                for item in &parsed.items {
                    let Item::Static(item) = item else { continue };
                    let Expr::Struct(value) = item.expr.as_ref() else {
                        continue;
                    };
                    for field in &value.fields {
                        let Member::Named(name) = &field.member else {
                            continue;
                        };
                        let name = name.to_string();
                        let batch = name == "ability_batch";
                        if !batch && !matches!(name.as_str(), "closures" | "melee" | "ranged") {
                            continue;
                        }
                        let expr = match &field.expr {
                            Expr::Reference(r) => r.expr.as_ref(),
                            e => e,
                        };
                        let entries = match expr {
                            Expr::Array(a) => a.elems.iter().collect::<Vec<_>>(),
                            e => vec![e],
                        };
                        for expr in entries {
                            let Expr::Path(function) = expr else {
                                return Err("Expected a generated function path".into());
                            };
                            let function_name =
                                function.path.segments.last().unwrap().ident.to_string();
                            let (variant, function_path) = if function_name == "zero" {
                                ("Zero".into(), "crate::libgen::zero".into())
                            } else {
                                (
                                    format!("{group}_{function_name}").to_upper_camel_case(),
                                    format!("crate::libgen::{group}::{module}::{function_name}"),
                                )
                            };
                            let ty = if batch { "AbilityBatch" } else { "Closure" };
                            let registry = if batch { &mut batches } else { &mut formulas };
                            if let Some(previous) =
                                registry.insert(variant.clone(), function_path.clone())
                            {
                                assert_eq!(previous, function_path, "Callable name collision");
                            }
                            replacements.push((
                                expr.span().byte_range(),
                                format!("crate::{ty}::{variant}"),
                            ));
                        }
                    }
                }
                let mut source = source;
                replacements.sort_by_key(|(range, _)| range.start);
                for (range, value) in replacements.into_iter().rev() {
                    source.replace_range(range, &value);
                }
                std::fs::write(path, source)?;
            }
        }
        let mut source = String::new();
        emit(
            &mut source,
            "Closure",
            &formulas,
            "'a",
            "(&'a tutorlolv2_types::Ctx,)",
            "f32",
            "(ctx,)",
            "ctx",
        )?;
        emit(
            &mut source,
            "AbilityBatch",
            &batches,
            "'a, 'b, 'c",
            "(&'a tutorlolv2_types::Ctx, crate::model::Modifiers, &'b mut crate::model::RangeDamage, &'c mut [i32])",
            "()",
            "(ctx, modifiers, onhit, out)",
            "ctx, modifiers, onhit, out",
        )?;
        std::fs::write(self.out.join("callables.rs"), source)?;
        Ok(())
    }
}

fn emit(
    out: &mut String,
    name: &str,
    entries: &BTreeMap<String, String>,
    lifetimes: &str,
    args: &str,
    result: &str,
    pattern: &str,
    call: &str,
) -> MayFail {
    writeln!(
        out,
        "#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)] pub enum {name} {{ {} }}",
        entries.keys().cloned().collect::<Vec<_>>().join(",")
    )?;
    writeln!(
        out,
        "const impl<{lifetimes}> core::ops::FnOnce<{args}> for {name} {{ type Output = {result}; extern \"rust-call\" fn call_once(self, {pattern}: {args}) -> {result} {{ match self {{"
    )?;
    for (variant, path) in entries {
        writeln!(out, "Self::{variant} => {path}({call}),")?;
    }
    writeln!(out, "}} }} }}")?;
    for (tr, method, receiver) in [("FnMut", "call_mut", "&mut self"), ("Fn", "call", "&self")] {
        writeln!(
            out,
            "const impl<{lifetimes}> core::ops::{tr}<{args}> for {name} {{ extern \"rust-call\" fn {method}({receiver}, args: {args}) -> {result} {{ core::ops::FnOnce::call_once(*self, args) }} }}"
        )?;
    }
    Ok(())
}
