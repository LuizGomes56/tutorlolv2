use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
    sync::LazyLock,
};

pub struct Batch {
    pub eval: String,
    pub fmt: String,
}

#[derive(Deserialize, Serialize)]
pub struct FmtArgs<'a, T> {
    pub target: &'a str,
    pub variant: &'a str,
    pub meta: T,
    pub replace: HashMap<&'a str, &'a str>,
    pub default: bool,
}

pub struct FmtOutput<'a> {
    pub html_range: Range<usize>,
    pub html: String,
    pub json: FmtArgs<'a, Value>,
    pub delete_range: Range<usize>,
}

pub fn batch<'b>(src: &'b str) -> BTreeMap<&'b str, BTreeMap<&'b str, Vec<FmtOutput<'b>>>> {
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

            let json = serde_json::from_str::<FmtArgs<'_, Value>>(inner).unwrap();

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

            let block = rest[..end].trim();
            let html = tutorlolv2_fmt::rust_html(block);

            let mut absolute_end = attr_end + end;

            if rest.get(end..end + 1) == Some(";") {
                absolute_end += 1;
            }

            FmtOutput {
                delete_range: start_index..absolute_end,
                html,
                html_range: 0..0,
                json,
            }
        })
        .collect::<Vec<_>>();

    let mut map = BTreeMap::<_, BTreeMap<_, Vec<_>>>::new();

    for data in result {
        let key = data.json.variant;
        map.entry(key)
            .or_default()
            .entry(data.json.target)
            .or_default()
            .push(data);
    }

    map
}
