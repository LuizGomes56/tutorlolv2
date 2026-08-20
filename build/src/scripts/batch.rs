use crate::{
    generators::utils::Tag,
    libfmt::{self, Builder},
};
use heck::ToSnakeCase;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, ops::Range, sync::LazyLock};
use tutorlolv2_codec::{EntityKind, FormulaDbBuilder, FormulaSource};
use tutorlolv2_types::AbilityId;
use tutorlolv2_wiki::MayFail;

#[derive(Debug, Deserialize, Serialize)]
pub struct FmtArgs<T> {
    pub target: String,
    pub variant: String,
    pub meta: T,
    pub default: bool,
}

#[derive(Debug, Serialize)]
pub struct FmtOutput {
    pub range: Range<usize>,
    pub builder: Builder,
    pub json: FmtArgs<Value>,
    pub delete_range: Range<usize>,
    block: String,
}

pub fn batch(src: String) -> BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>> {
    static FMT_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"#\[fmt\((\{.*?\})\)\]"#).unwrap());

    let result = FMT_RE
        .captures_iter(&src)
        .par_bridge()
        .map(|caps| {
            let full = caps.get(0).unwrap();
            let inner = caps.get(1).unwrap().as_str();

            let start_index = full.start();
            let attr_end = full.end();

            let json = serde_json::from_str::<FmtArgs<Value>>(inner).unwrap();

            let rest = &src[attr_end..];
            let start = rest.find('{').unwrap();

            let mut depth = 0;
            let mut end = 0;

            for (i, ch) in rest[start..].char_indices() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;

                        if depth == 0 {
                            end = start + i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let block = rest[..end].trim().to_string();
            let builder = libfmt::rust_html(&block);

            let mut absolute_end = attr_end + end;

            if rest.get(end..end + 1) == Some(";") {
                absolute_end += 1;
            }

            FmtOutput {
                delete_range: start_index..absolute_end,
                builder,
                range: 0..0,
                json,
                block,
            }
        })
        .collect::<Vec<_>>();

    let mut map = BTreeMap::<_, BTreeMap<_, Vec<_>>>::new();

    for data in result {
        map.entry(data.json.variant.clone())
            .or_default()
            .entry(data.json.target.clone())
            .or_default()
            .push(data);
    }

    map
}

pub fn pack_formulas(
    packer: &mut FormulaDbBuilder,
    tag: Tag,
    result: &BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>>,
) -> MayFail {
    for (i, (variant, inner_map)) in result.into_iter().enumerate() {
        let Some(outputs) = inner_map.get("formula") else {
            continue;
        };

        let id = variant.to_snake_case();

        let (formulas, refs) = outputs
            .iter()
            .filter_map(
                |FmtOutput {
                     block,
                     json: FmtArgs { meta, .. },
                     ..
                 }| {
                    let body = block
                        .find('{')
                        .map(|pos| block[pos + 1..].trim().trim_end_matches('}').trim())?;

                    let (source, local, fn_name) = match tag {
                        Tag::Champions => {
                            let (f, k) =
                                serde_json::from_value::<(AbilityId, u8)>(meta.clone()).ok()?;
                            let fn_type = f.discriminant();
                            (
                                body.rfind("] */").map(|pos| &body[pos + 4..])?,
                                k,
                                format!("{id}_{fn_type}").to_lowercase(),
                            )
                        }
                        Tag::Items | Tag::Runes => {
                            let k = meta.as_u64()?;
                            (body, k as _, format!("{id}_f{k}").to_lowercase())
                        }
                    };

                    Some((FormulaSource { local, source }, (fn_name, local)))
                },
            )
            .unzip::<_, _, Vec<_>, _>();

        match tag {
            Tag::Champions => {
                /* comment[{comment:?}] */
                /* name[{name:?}] */
                packer.push_champion(i as _, &formulas, &refs)
            }
            _ => {
                let entity = match tag {
                    Tag::Items => EntityKind::Item,
                    Tag::Runes => EntityKind::Rune,
                    _ => unreachable!(),
                };

                packer.push_sparse(entity, i as _, &formulas, &refs)
            }
        }
        .map_err(|e| format!("Packer Error [{id}][formulas={formulas:?}][refs={refs:?}]: {e:?}"))?;
    }

    Ok(())
}
