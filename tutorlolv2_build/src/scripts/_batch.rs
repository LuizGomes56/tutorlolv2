use crate::Tracker;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    sync::LazyLock,
};
use tutorlolv2_dev::MayFail;
use tutorlolv2_fmt::{pascal_case, to_ssnake};

pub fn simplify(formula: &str) -> String {
    symb_anafis::simplify(&formula.replace("ctx.", "ctx_"), &[], None)
        .map(|r| r.replace("ctx_", "ctx."))
        .unwrap_or(formula.to_string())
}

pub fn slice_repr<T: Debug>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|ident| format!("&{ident:#?}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn get_arg(len: usize, i: &usize) -> &dyn Display {
    match *i {
        i if i == 0 && i == len - 1 => &"unique",
        i if i == len - 1 => &"last",
        i if i == 0 => &"first",
        _ => i,
    }
}

pub fn get_aliases<'a>(id: &'a str, name: &'a str) -> Vec<String> {
    let get = |s: &str| {
        [
            s.to_string(),
            s.to_lowercase(),
            s.to_uppercase(),
            pascal_case(s),
            pascal_case(s).to_lowercase(),
            pascal_case(s).to_uppercase(),
            to_ssnake(s),
            to_ssnake(s).to_lowercase(),
            to_ssnake(s).to_uppercase(),
        ]
    };

    [get(id), get(name)].concat()
}

pub struct Batch {
    pub eval: String,
    pub fmt: String,
}

#[derive(Debug)]
struct FmtBatchError(String);

impl core::fmt::Display for FmtBatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for FmtBatchError {}

fn fmt_err<T>(msg: impl Into<String>) -> MayFail<T> {
    Err(Box::new(FmtBatchError(msg.into())))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ArrayFmt {
    First,
    Last,
    Unique,
    Index(usize),
}

#[derive(Clone, Debug)]
struct Replacement {
    from: String,
    to: String,
}

#[derive(Clone, Debug)]
struct FmtAttr {
    target: String,
    variant: String,
    keep: bool,
    array: Option<ArrayFmt>,
    replacements: Vec<Replacement>,
}

#[derive(Clone, Debug)]
struct FmtEvent {
    target: String,
    variant: String,
    array: Option<ArrayFmt>,
    range: String,
}

#[derive(Clone, Debug)]
struct TargetOut {
    name: String,
    out: String,
    is_array_target: bool,
}

impl<'a> Tracker<'a> {
    pub fn record_fmt_html(&mut self, value: &str, replacements: &[Replacement]) -> String {
        let mut value = value.to_owned();

        for replacement in replacements {
            value = value.replace(&replacement.from, &replacement.to);
        }

        let html = tutorlolv2_fmt::rust_html(value.trim());

        let start = self.offset();
        self.inner.push_str(&html);
        let end = self.offset();

        format!("{start}..{end},")
    }
}

pub fn fmt_batch(
    tracker: &mut Tracker<'_>,
    src: String,
    fmt_args: Vec<(&str, String)>,
) -> MayFail<String> {
    static FMT_ATTR_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?s)#\s*\[\s*fmt\s*\((?P<args>.*?)\)\s*\]"#).unwrap());

    let src = tutorlolv2_fmt::rustfmt(&src, None);

    let mut targets = fmt_args
        .into_iter()
        .map(|(name, out)| TargetOut {
            name: name.to_owned(),
            is_array_target: out.contains("&["),
            out,
        })
        .collect::<Vec<_>>();

    let mut events = Vec::<FmtEvent>::new();
    let mut variants = Vec::<String>::new();

    let mut final_src = String::with_capacity(src.len());
    let mut search_from = 0;
    let mut copy_from = 0;

    while let Some(caps) = FMT_ATTR_RE.captures_at(&src, search_from) {
        let whole = caps.get(0).unwrap();
        let args = caps.name("args").unwrap().as_str();

        let attr_start = whole.start();
        let attr_end = whole.end();

        let attr = parse_fmt_args(args)?;

        if !targets.iter().any(|target| target.name == attr.target) {
            return fmt_err(format!("unknown fmt target `{}`", attr.target));
        }

        if !variants.iter().any(|variant| variant == &attr.variant) {
            variants.push(attr.variant.clone());
        }

        let capture_end = find_capture_end(&src, attr_end)?;
        let captured = &src[attr_end..capture_end];

        let range = tracker.record_fmt_html(captured, &attr.replacements);

        events.push(FmtEvent {
            target: attr.target.clone(),
            variant: attr.variant.clone(),
            array: attr.array.clone(),
            range,
        });

        final_src.push_str(&src[copy_from..attr_start]);

        if attr.keep {
            // Remove somente o atributo #[fmt(...)].
            copy_from = attr_end;
        } else {
            // Remove o atributo #[fmt(...)] + o item capturado.
            copy_from = capture_end;
        }

        search_from = capture_end;
    }

    final_src.push_str(&src[copy_from..]);

    // Se algum target recebeu array(...), ele é um target de arrays,
    // mesmo que a assinatura inicial não tenha sido detectada por `&[`.
    for target in &mut targets {
        if events
            .iter()
            .any(|event| event.target == target.name && event.array.is_some())
        {
            target.is_array_target = true;
        }
    }

    for target in &mut targets {
        for variant in &variants {
            let variant_events: Vec<&FmtEvent> = events
                .iter()
                .filter(|event| event.target == target.name && event.variant == *variant)
                .collect();

            if target.is_array_target {
                push_array_variant(&mut target.out, &target.name, variant, &variant_events)?;
            } else {
                push_scalar_variant(&mut target.out, &target.name, variant, &variant_events)?;
            }
        }

        target.out.push_str("];");
    }

    for target in targets {
        final_src.push_str("\n\n");
        final_src.push_str(&target.out);
    }

    Ok(tutorlolv2_fmt::rustfmt(&final_src, None))
}

fn parse_fmt_args(args: &str) -> MayFail<FmtAttr> {
    let parts = split_top_level_commas(args);

    let Some((target, rest)) = parts.split_first() else {
        return fmt_err("empty #[fmt(...)]");
    };

    let mut attr = FmtAttr {
        target: target.trim().to_owned(),
        variant: String::new(),
        keep: false,
        array: None,
        replacements: Vec::new(),
    };

    for part in rest {
        let part = part.trim();

        if part.is_empty() {
            continue;
        }

        if part == "keep" {
            attr.keep = true;
            continue;
        }

        if part.starts_with("remove(") {
            return fmt_err("remove(...) was removed; use replace(\"from\", \"\") instead");
        }

        if let Some(value) = call_arg(part, "variant") {
            attr.variant = value.trim().to_owned();
            continue;
        }

        if let Some(value) = call_arg(part, "array") {
            attr.array = Some(parse_array_fmt(value.trim())?);
            continue;
        }

        if let Some(value) = call_arg(part, "replace") {
            attr.replacements.push(parse_replacement(value)?);
            continue;
        }

        return fmt_err(format!("unknown #[fmt] argument `{part}`"));
    }

    if attr.target.is_empty() {
        return fmt_err("fmt target cannot be empty");
    }

    if attr.variant.is_empty() {
        return fmt_err(format!(
            "fmt target `{}` is missing variant(...)",
            attr.target
        ));
    }

    Ok(attr)
}

fn parse_replacement(value: &str) -> MayFail<Replacement> {
    let parts = split_top_level_commas(value);

    if parts.len() != 2 {
        return fmt_err(format!(
            "replace(...) expects exactly 2 arguments: replace(\"from\", \"to\"), got `{value}`"
        ));
    }

    let from = parse_string_literal(parts[0].trim())?;
    let to = parse_replacement_to(parts[1].trim())?;

    Ok(Replacement { from, to })
}

fn parse_replacement_to(value: &str) -> MayFail<String> {
    let value = value.trim();

    if value.starts_with('"') || value.starts_with('r') {
        parse_string_literal(value)
    } else {
        // Permite replace("old", new), tratando `new` como texto literal "new".
        Ok(value.to_owned())
    }
}

fn parse_array_fmt(value: &str) -> MayFail<ArrayFmt> {
    match value {
        "first" => Ok(ArrayFmt::First),
        "last" => Ok(ArrayFmt::Last),
        "unique" => Ok(ArrayFmt::Unique),
        _ => match value.parse::<usize>() {
            Ok(index) => Ok(ArrayFmt::Index(index)),
            Err(_) => fmt_err(format!("invalid array(...) value `{value}`")),
        },
    }
}

fn call_arg<'a>(part: &'a str, name: &str) -> Option<&'a str> {
    let rest = part.trim().strip_prefix(name)?.trim_start();

    if !rest.starts_with('(') || !rest.ends_with(')') {
        return None;
    }

    Some(&rest[1..rest.len() - 1])
}

fn parse_string_literal(value: &str) -> MayFail<String> {
    let value = value.trim();

    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        return Ok(unescape_basic_string(&value[1..value.len() - 1]));
    }

    if value.starts_with('r') {
        let bytes = value.as_bytes();
        let mut quote = 1;

        while quote < bytes.len() && bytes[quote] == b'#' {
            quote += 1;
        }

        if quote < bytes.len() && bytes[quote] == b'"' {
            let hashes = &value[1..quote];
            let end_pattern = format!("\"{hashes}");

            if value.ends_with(&end_pattern) {
                let start = quote + 1;
                let end = value.len() - end_pattern.len();

                return Ok(value[start..end].to_owned());
            }
        }
    }

    fmt_err(format!("expected string literal, got `{value}`"))
}

fn unescape_basic_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('0') => out.push('\0'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }

    out
}

fn push_scalar_variant(
    out: &mut String,
    target: &str,
    variant: &str,
    events: &[&FmtEvent],
) -> MayFail {
    match events {
        [] => {
            out.push_str("0..0,");
            Ok(())
        }
        [event] => {
            if event.array.is_some() {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` received array(...) but target is scalar"
                ));
            }

            out.push_str(&event.range);
            Ok(())
        }
        _ => fmt_err(format!(
            "target `{target}` for variant `{variant}` received multiple scalar entries"
        )),
    }
}

fn push_array_variant(
    out: &mut String,
    target: &str,
    variant: &str,
    events: &[&FmtEvent],
) -> MayFail {
    if events.is_empty() {
        out.push_str("&[],");
        return Ok(());
    }

    if events.len() == 1 {
        let event = events[0];

        match event.array {
            Some(ArrayFmt::Unique) => {
                out.push_str("&[");
                out.push_str(&event.range);
                out.push_str("],");
                return Ok(());
            }
            Some(ArrayFmt::First) => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` has array(first) but no array(last)"
                ));
            }
            Some(ArrayFmt::Last) => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` has array(last) without array(first); use array(unique)"
                ));
            }
            Some(ArrayFmt::Index(_)) => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` starts with array(number) without array(first)"
                ));
            }
            None => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` is array target but entry has no array(...)"
                ));
            }
        }
    }

    let mut open = false;

    for event in events {
        match event.array {
            Some(ArrayFmt::First) => {
                if open {
                    return fmt_err(format!(
                        "target `{target}` for variant `{variant}` opened array twice"
                    ));
                }

                out.push_str("&[");
                out.push_str(&event.range);
                open = true;
            }
            Some(ArrayFmt::Index(_)) => {
                if !open {
                    return fmt_err(format!(
                        "target `{target}` for variant `{variant}` has array(number) before array(first)"
                    ));
                }

                out.push_str(&event.range);
            }
            Some(ArrayFmt::Last) => {
                if !open {
                    return fmt_err(format!(
                        "target `{target}` for variant `{variant}` has array(last) before array(first)"
                    ));
                }

                out.push_str(&event.range);
                out.push_str("],");
                open = false;
            }
            Some(ArrayFmt::Unique) => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` has array(unique) mixed with other entries"
                ));
            }
            None => {
                return fmt_err(format!(
                    "target `{target}` for variant `{variant}` is array target but entry has no array(...)"
                ));
            }
        }
    }

    if open {
        return fmt_err(format!(
            "target `{target}` for variant `{variant}` has unclosed array(first); missing array(last)"
        ));
    }

    Ok(())
}

fn split_top_level_commas(src: &str) -> Vec<&str> {
    let bytes = src.as_bytes();
    let mut parts = Vec::new();

    let mut start = 0;
    let mut i = 0;
    let mut depth = 0usize;

    while i < bytes.len() {
        if let Some(next) = skip_literal_or_comment(src, i) {
            i = next;
            continue;
        }

        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                parts.push(&src[start..i]);
                start = i + 1;
            }
            _ => {}
        }

        i += 1;
    }

    parts.push(&src[start..]);
    parts
}

fn find_capture_end(src: &str, attr_end: usize) -> MayFail<usize> {
    let Some(open) = find_next_code_byte(src, attr_end, b'{') else {
        return fmt_err("fmt item has no following `{`");
    };

    let Some(close) = find_matching_brace(src, open) else {
        return fmt_err("fmt item has unclosed `{`");
    };

    let after = skip_ws(src, close + 1);

    if src.as_bytes().get(after) == Some(&b';') {
        Ok(after + 1)
    } else {
        Ok(close + 1)
    }
}

fn find_next_code_byte(src: &str, mut i: usize, needle: u8) -> Option<usize> {
    let bytes = src.as_bytes();

    while i < bytes.len() {
        if let Some(next) = skip_literal_or_comment(src, i) {
            i = next;
            continue;
        }

        if bytes[i] == needle {
            return Some(i);
        }

        i += 1;
    }

    None
}

fn find_matching_brace(src: &str, open: usize) -> Option<usize> {
    let bytes = src.as_bytes();

    let mut i = open;
    let mut depth = 0usize;

    while i < bytes.len() {
        if let Some(next) = skip_literal_or_comment(src, i) {
            i = next;
            continue;
        }

        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;

                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }

        i += 1;
    }

    None
}

fn skip_literal_or_comment(src: &str, i: usize) -> Option<usize> {
    let bytes = src.as_bytes();

    match bytes.get(i).copied()? {
        b'"' => Some(skip_normal_string(src, i)),
        b'\'' => try_skip_char_literal(src, i),
        b'/' if bytes.get(i + 1) == Some(&b'/') => Some(skip_line_comment(src, i)),
        b'/' if bytes.get(i + 1) == Some(&b'*') => Some(skip_block_comment(src, i)),
        b'r' => try_skip_raw_string(src, i),
        b'b' if bytes.get(i + 1) == Some(&b'"') => Some(skip_normal_string(src, i + 1)),
        b'b' if bytes.get(i + 1) == Some(&b'r') => try_skip_raw_string(src, i + 1),
        _ => None,
    }
}

fn skip_normal_string(src: &str, quote: usize) -> usize {
    let bytes = src.as_bytes();
    let mut i = quote + 1;

    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i += 2,
            b'"' => return i + 1,
            _ => i += 1,
        }
    }

    bytes.len()
}

fn try_skip_raw_string(src: &str, r_pos: usize) -> Option<usize> {
    let bytes = src.as_bytes();

    if bytes.get(r_pos) != Some(&b'r') {
        return None;
    }

    let mut quote = r_pos + 1;

    while quote < bytes.len() && bytes[quote] == b'#' {
        quote += 1;
    }

    if bytes.get(quote) != Some(&b'"') {
        return None;
    }

    let hashes = &src[r_pos + 1..quote];
    let end_pattern = format!("\"{hashes}");

    let body_start = quote + 1;
    let rel_end = src[body_start..].find(&end_pattern)?;

    Some(body_start + rel_end + end_pattern.len())
}

fn try_skip_char_literal(src: &str, quote: usize) -> Option<usize> {
    let bytes = src.as_bytes();
    let mut i = quote + 1;

    if i >= bytes.len() || bytes[i] == b'\n' || bytes[i] == b'\r' {
        return None;
    }

    if bytes[i] == b'\\' {
        i += 2;
    } else {
        let ch = src[i..].chars().next()?;
        i += ch.len_utf8();
    }

    if bytes.get(i) == Some(&b'\'') {
        Some(i + 1)
    } else {
        None
    }
}

fn skip_line_comment(src: &str, slash: usize) -> usize {
    let bytes = src.as_bytes();
    let mut i = slash + 2;

    while i < bytes.len() && bytes[i] != b'\n' {
        i += 1;
    }

    i
}

fn skip_block_comment(src: &str, slash: usize) -> usize {
    let bytes = src.as_bytes();
    let mut i = slash + 2;
    let mut depth = 1usize;

    while i + 1 < bytes.len() {
        match (bytes[i], bytes[i + 1]) {
            (b'/', b'*') => {
                depth += 1;
                i += 2;
            }
            (b'*', b'/') => {
                depth -= 1;
                i += 2;

                if depth == 0 {
                    return i;
                }
            }
            _ => i += 1,
        }
    }

    bytes.len()
}

fn skip_ws(src: &str, mut i: usize) -> usize {
    let bytes = src.as_bytes();

    while i < bytes.len() && matches!(bytes[i], b' ' | b'\n' | b'\r' | b'\t') {
        i += 1;
    }

    i
}
