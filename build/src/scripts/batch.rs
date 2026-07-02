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
    pub html_range: Range<usize>,
    pub html: String,
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

            let formatter = if json.target == "json" {
                tutorlolv2_fmt::json_html
            } else {
                tutorlolv2_fmt::rust_html
            };

            let html = formatter(&block);

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
        map.entry(data.json.variant.clone())
            .or_default()
            .entry(data.json.target.clone())
            .or_default()
            .push(data);
    }

    map
}

pub struct Tracker<'a> {
    inner: &'a mut String,
}

impl<'a> Tracker<'a> {
    /// Creates a new instance of self, from an existing string that
    /// should live longer than this struct
    pub const fn new(inner: &'a mut String) -> Self {
        Self { inner }
    }

    /// Get the current length of the string, which represents
    /// the `end` offset of the last record
    pub const fn offset(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, value: &str) -> Range<usize> {
        if let Some(pos) = self.inner.find(value) {
            return pos..pos + value.len();
        }

        let start = self.offset();
        self.inner.push_str(value);
        start..self.offset()
    }

    pub fn batch(&mut self, batch: &mut BTreeMap<String, BTreeMap<String, Vec<FmtOutput>>>) {
        for value in batch.values_mut() {
            for data in value.values_mut() {
                for output in data.iter_mut() {
                    if !output.json.default {
                        output.html_range = self.push(&output.html);
                    }
                }
            }
        }
    }
}
