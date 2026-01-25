use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
    time::SystemTime,
};
use syn::{LitStr, Token, parse::Parse, parse_macro_input, spanned::Spanned};

struct ExpandDirArgs {
    dir: LitStr,
    _comma: Token![,],
    _bar1: Token![|],
    placeholder: Placeholder,
    _bar2: Token![|],
    template: TokenStream2,
}

enum Placeholder {
    Simple(Ident),
    Array(Ident),
}

impl Parse for ExpandDirArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let dir: LitStr = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let _bar1: Token![|] = input.parse()?;

        let placeholder = if input.peek(syn::token::Bracket) {
            let content;
            let _ = syn::bracketed!(content in input);
            let name: Ident = content.parse()?;
            if !content.is_empty() {
                return Err(syn::Error::new(
                    content.span(),
                    "expected single ident inside []",
                ));
            }
            Placeholder::Array(name)
        } else {
            Placeholder::Simple(input.parse()?)
        };

        let _bar2: Token![|] = input.parse()?;
        let template: TokenStream2 = input.parse()?;

        Ok(Self {
            dir,
            _comma,
            _bar1,
            placeholder,
            _bar2,
            template,
        })
    }
}

#[proc_macro]
pub fn expand_dir(input: TokenStream) -> TokenStream {
    let ExpandDirArgs {
        dir,
        placeholder,
        template,
        ..
    } = parse_macro_input!(input as ExpandDirArgs);

    let dir_str = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), dir.value());

    let entries = match fs::read_dir(&dir_str) {
        Ok(rd) => rd,
        Err(_) => {
            return syn::Error::new(dir.span(), "failed to read directory")
                .to_compile_error()
                .into();
        }
    };

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for ent in entries.flatten() {
        let path = ent.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                files.push((stem.to_string(), path));
            }
        }
    }

    if files.is_empty() {
        return syn::Error::new(dir.span(), "no .json files found in directory")
            .to_compile_error()
            .into();
    }

    let (ph_ident, is_array) = match &placeholder {
        Placeholder::Simple(id) => (id.clone(), false),
        Placeholder::Array(id) => (id.clone(), true),
    };

    let tpl = unwrap_outer_group(template);

    let compiled = match compile_template(tpl, &ph_ident) {
        Ok(c) => c,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut pieces: Vec<TokenStream2> = Vec::with_capacity(files.len());

    for (stem, path) in files {
        let id = Ident::new(&stem, Span::call_site());

        let json: Option<Arc<serde_json::Value>> = if compiled.needs_json {
            match load_json_cached(&path, dir.span()) {
                Ok(v) => Some(v),
                Err(e) => return e.to_compile_error().into(),
            }
        } else {
            None
        };

        let rendered = match render_compiled(&compiled, &ph_ident, &stem, json.as_deref()) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        };

        let final_ts = subst_ident(rendered, &ph_ident, &id);
        pieces.push(final_ts);
    }

    let out = if is_array {
        quote! { [#( #pieces ),*] }
    } else {
        quote! { #( #pieces )* }
    };

    TokenStream::from(out)
}

fn unwrap_outer_group(ts: TokenStream2) -> TokenStream2 {
    let mut it = ts.clone().into_iter();
    if let (Some(TokenTree::Group(g)), None) = (it.next(), it.next()) {
        return g.stream();
    }
    ts
}

enum Node {
    TT(TokenTree),
    Group {
        delim: Delimiter,
        span: Span,
        children: Vec<Node>,
    },
    Directive {
        span: Span,
        expr: syn::Expr,
        needs_json: bool,
    },
}

struct CompiledTemplate {
    nodes: Vec<Node>,
    needs_json: bool,
    root_ident: Ident,
}

fn compile_template(ts: TokenStream2, root_ident: &Ident) -> syn::Result<CompiledTemplate> {
    fn compile_nodes(ts: TokenStream2, root_ident: &Ident) -> syn::Result<Vec<Node>> {
        let mut out = Vec::new();
        let mut it = ts.into_iter().peekable();

        while let Some(tt) = it.next() {
            match tt {
                TokenTree::Group(g) => {
                    let children = compile_nodes(g.stream(), root_ident)?;
                    out.push(Node::Group {
                        delim: g.delimiter(),
                        span: g.span(),
                        children,
                    });
                }

                TokenTree::Punct(p) if p.as_char() == '%' => {
                    let start_span = p.span();
                    let mut inner = TokenStream2::new();
                    let mut closed = false;

                    while let Some(nxt) = it.next() {
                        if let TokenTree::Punct(p2) = &nxt {
                            if p2.as_char() == '%' {
                                closed = true;
                                break;
                            }
                        }
                        inner.extend([nxt]);
                    }

                    if !closed {
                        return Err(syn::Error::new(start_span, "unterminated %...% directive"));
                    }

                    let expr: syn::Expr = syn::parse2(inner)?;
                    let needs_json = expr_contains_field_access(&expr);

                    out.push(Node::Directive {
                        span: start_span,
                        expr,
                        needs_json,
                    });
                }

                other => out.push(Node::TT(other)),
            }
        }

        Ok(out)
    }

    let nodes = compile_nodes(ts, root_ident)?;
    let needs_json = nodes_need_json(&nodes);

    Ok(CompiledTemplate {
        nodes,
        needs_json,
        root_ident: root_ident.clone(),
    })
}

fn nodes_need_json(nodes: &[Node]) -> bool {
    nodes.iter().any(|n| match n {
        Node::Directive { needs_json, .. } => *needs_json,
        Node::Group { children, .. } => nodes_need_json(children),
        _ => false,
    })
}

fn expr_contains_field_access(expr: &syn::Expr) -> bool {
    use syn::Expr;
    match expr {
        Expr::Field(f) => expr_contains_field_access(&f.base) || true,
        Expr::Call(c) => {
            expr_contains_field_access(&c.func) || c.args.iter().any(expr_contains_field_access)
        }
        Expr::Paren(p) => expr_contains_field_access(&p.expr),
        Expr::Path(_) => false,
        Expr::Lit(_) => false,
        Expr::Unary(u) => expr_contains_field_access(&u.expr),
        Expr::Binary(b) => {
            expr_contains_field_access(&b.left) || expr_contains_field_access(&b.right)
        }
        Expr::MethodCall(m) => {
            expr_contains_field_access(&m.receiver) || m.args.iter().any(expr_contains_field_access)
        }
        Expr::Index(i) => {
            expr_contains_field_access(&i.expr) || expr_contains_field_access(&i.index)
        }
        Expr::Reference(r) => expr_contains_field_access(&r.expr),
        Expr::Cast(c) => expr_contains_field_access(&c.expr),
        Expr::Block(b) => b.block.stmts.iter().any(|s| match s {
            syn::Stmt::Local(syn::Local { init: Some(i), .. }) => {
                expr_contains_field_access(&i.expr)
            }
            syn::Stmt::Expr(e, _) => expr_contains_field_access(e),
            syn::Stmt::Local(_) => false,
            syn::Stmt::Item(_) => false,
            syn::Stmt::Macro(_) => false,
        }),
        _ => false,
    }
}

fn render_compiled(
    compiled: &CompiledTemplate,
    root_ident: &Ident,
    stem: &str,
    json: Option<&serde_json::Value>,
) -> syn::Result<TokenStream2> {
    render_nodes(&compiled.nodes, root_ident, stem, json)
}

fn render_nodes(
    nodes: &[Node],
    root_ident: &Ident,
    stem: &str,
    json: Option<&serde_json::Value>,
) -> syn::Result<TokenStream2> {
    let mut out = TokenStream2::new();

    for n in nodes {
        match n {
            Node::TT(tt) => out.extend([tt.clone()]),

            Node::Group {
                delim,
                span,
                children,
            } => {
                let inner = render_nodes(children, root_ident, stem, json)?;
                let mut g = Group::new(*delim, inner);
                g.set_span(*span);
                out.extend([TokenTree::Group(g)]);
            }

            Node::Directive {
                span,
                expr,
                needs_json,
            } => {
                if *needs_json && json.is_none() {
                    return Err(syn::Error::new(
                        *span,
                        "directive requires JSON fields (File.x), but JSON was not loaded",
                    ));
                }

                let rendered = eval_expr_to_string(expr, root_ident, stem, json)
                    .map_err(|msg| syn::Error::new(expr.span(), msg))?;

                let injected = inject_tokens_from_string(&rendered, *span)?;
                out.extend(injected);
            }
        }
    }

    Ok(out)
}

fn inject_tokens_from_string(s: &str, span: Span) -> syn::Result<TokenStream2> {
    if syn::parse_str::<syn::Ident>(s).is_ok() {
        let ident = if is_keyword(s) {
            Ident::new_raw(s, span)
        } else {
            Ident::new(s, span)
        };
        return Ok(ident.to_token_stream());
    }

    if syn::parse_str::<syn::LitInt>(s).is_ok() {
        let ts: TokenStream2 = s.parse().map_err(|e| {
            syn::Error::new(
                span,
                format!("failed to parse integer literal as tokens: {e}"),
            )
        })?;
        return Ok(ts);
    }

    let ts: TokenStream2 = s.parse().map_err(|e| {
        syn::Error::new(
            span,
            format!("directive produced invalid Rust tokens: {e} (output: {s:?})"),
        )
    })?;
    Ok(ts)
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn eval_expr_to_string(
    expr: &syn::Expr,
    root_ident: &Ident,
    stem: &str,
    json: Option<&serde_json::Value>,
) -> Result<String, String> {
    use syn::Expr;

    match expr {
        Expr::Paren(p) => eval_expr_to_string(&p.expr, root_ident, stem, json),

        Expr::Lit(lit) => {
            if let syn::Lit::Str(s) = &lit.lit {
                Ok(s.value())
            } else {
                Ok(lit.lit.to_token_stream().to_string())
            }
        }

        Expr::Call(call) => {
            let func = match &*call.func {
                Expr::Path(p) if p.path.segments.len() == 1 => p.path.segments[0].ident.to_string(),
                _ => return Err("only simple function calls like snake(...) are supported".into()),
            };

            if call.args.len() != 1 {
                return Err(format!("{func}(...) must have exactly 1 argument"));
            }

            let arg = eval_expr_to_string(&call.args[0], root_ident, stem, json)?;

            match func.as_str() {
                "snake" => Ok(tutorlolv2_fmt::to_ssnake(&arg).to_lowercase()),
                "pascal" => Ok(tutorlolv2_fmt::pascal_case(&arg)),
                _ => Err(format!("unknown directive function: {func}")),
            }
        }

        Expr::Field(_) => {
            let json = json.ok_or_else(|| "JSON not loaded".to_string())?;
            let v = resolve_json_path(expr, root_ident, json)?;
            Ok(json_value_to_string(v)?)
        }

        Expr::Path(p) => {
            if p.path.segments.len() == 1 && p.path.segments[0].ident == *root_ident {
                Ok(stem.to_string())
            } else {
                Err("unsupported path; use File.<field> or snake(File.<field>)".into())
            }
        }

        _ => Err("unsupported directive expression".into()),
    }
}

fn resolve_json_path<'a>(
    expr: &syn::Expr,
    root_ident: &Ident,
    json: &'a serde_json::Value,
) -> Result<&'a serde_json::Value, String> {
    let mut keys: Vec<String> = Vec::new();
    let mut cur = expr;

    loop {
        match cur {
            syn::Expr::Field(f) => {
                let member = match &f.member {
                    syn::Member::Named(id) => id.to_string(),
                    syn::Member::Unnamed(_) => return Err("tuple fields not supported".into()),
                };
                keys.push(member);
                cur = &f.base;
            }
            syn::Expr::Path(p) => {
                if p.path.segments.len() == 1 && p.path.segments[0].ident == *root_ident {
                    break;
                }
                return Err(
                    "field access must start from the placeholder ident (ex.: File.name)".into(),
                );
            }
            syn::Expr::Paren(p) => cur = &p.expr,
            _ => return Err("unsupported JSON path expression".into()),
        }
    }

    keys.reverse();

    let mut v = json;
    for k in keys {
        v = v.get(&k).ok_or_else(|| format!("missing JSON key: {k}"))?;
    }

    Ok(v)
}

fn json_value_to_string(v: &serde_json::Value) -> Result<String, String> {
    let v = if let serde_json::Value::Object(map) = v {
        map.get("Value").unwrap_or(v)
    } else {
        v
    };

    match v {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Bool(b) => Ok(b.to_string()),
        serde_json::Value::Null => Err("value is null".into()),
        other => Ok(other.to_string()),
    }
}

static JSON_CACHE: OnceLock<Mutex<HashMap<PathBuf, (SystemTime, Arc<serde_json::Value>)>>> =
    OnceLock::new();

fn load_json_cached(path: &PathBuf, span: Span) -> syn::Result<Arc<serde_json::Value>> {
    let meta = fs::metadata(path)
        .map_err(|e| syn::Error::new(span, format!("failed to stat {}: {e}", path.display())))?;

    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    let cache = JSON_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    {
        let map = cache.lock().unwrap();
        if let Some((cached_mtime, v)) = map.get(path) {
            if *cached_mtime == mtime {
                return Ok(v.clone());
            }
        }
    }

    let bytes = fs::read(path)
        .map_err(|e| syn::Error::new(span, format!("failed to read {}: {e}", path.display())))?;

    let v: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| syn::Error::new(span, format!("invalid json in {}: {e}", path.display())))?;

    let v = Arc::new(v);

    let mut map = cache.lock().unwrap();
    map.insert(path.clone(), (mtime, v.clone()));

    Ok(v)
}

fn subst_ident(ts: TokenStream2, from: &Ident, to: &Ident) -> TokenStream2 {
    ts.into_iter()
        .map(|tt| match tt {
            TokenTree::Ident(id) if id == *from => TokenTree::Ident(to.clone()),
            TokenTree::Group(g) => {
                let inner = subst_ident(g.stream(), from, to);
                let mut ng = Group::new(g.delimiter(), inner);
                ng.set_span(g.span());
                TokenTree::Group(ng)
            }
            other => other,
        })
        .collect()
}
