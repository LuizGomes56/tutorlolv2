use crate::libfmt::{self, Builder, Op};
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    sync::LazyLock,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct FmtArgs<T> {
    pub target: String,
    pub variant: String,
    pub meta: T,
    pub replace: HashMap<String, String>,
    pub default: bool,
}

#[derive(Debug, Serialize)]
pub struct FmtOutput {
    pub range: Range<usize>,
    pub builder: Builder,
    pub json: FmtArgs<Value>,
    pub delete_range: Range<usize>,
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

            let mut block = rest[..end].trim().to_string();

            for (from, into) in &json.replace {
                block = block.replace(from, into);
            }

            let formatter = libfmt::rust_html;
            // formatter had the type: String because both functions returned HTML.
            // Now they return struct Builder with the IR
            // let formatter = if json.target == "json" {
            //     libfmt::json_html
            // } else {
            //     libfmt::rust_html
            // };
            //

            let builder = formatter(&block);

            let mut absolute_end = attr_end + end;

            if rest.get(end..end + 1) == Some(";") {
                absolute_end += 1;
            }

            FmtOutput {
                delete_range: start_index..absolute_end,
                builder,
                range: 0..0,
                json,
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
