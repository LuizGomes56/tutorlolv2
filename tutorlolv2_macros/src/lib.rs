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

// ------------------------------
// #[skip_if(...)] support
// ------------------------------

enum SkipIfAttr {
    Expr {
        expr: syn::Expr,
        needs_json: bool,
    },
    /// skip_if("<mods file>", |Mod| <bool expr>)
    ///
    /// The `<bool expr>` may reference:
    /// - the directory placeholder ident (ex.: `Name`) → current JSON stem string
    /// - the closure param ident (ex.: `Mod`) → module ident string from the mods file
    /// - string literals
    /// - simple calls: `pascal(...)` / `snake(...)`
    /// - boolean composition: `==`, `!=`, `&&`, `||`, parentheses
    /// - JSON fields like `Name.some_field == "x"` (same as directives)
    ModList {
        mods_path: LitStr,
        param_ident: Ident,
        expr: syn::Expr,
        needs_json: bool,
    },
}

struct SkipIfArgs {
    kind: SkipIfArgsKind,
}

enum SkipIfArgsKind {
    Expr(syn::Expr),
    ModList {
        mods_path: LitStr,
        param_ident: Ident,
        expr: syn::Expr,
    },
}

impl Parse for SkipIfArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        // Try parse: "<path>", |Param| <expr>
        if input.peek(LitStr) {
            let fork = input.fork();
            let _mods_path: LitStr = fork.parse()?;
            if fork.peek(Token![,]) {
                let _comma: Token![,] = fork.parse()?;
                if fork.peek(Token![|]) {
                    let _bar1: Token![|] = fork.parse()?;
                    let _param_ident: Ident = fork.parse()?;
                    let _bar2: Token![|] = fork.parse()?;
                    let _expr: syn::Expr = fork.parse()?;
                    if fork.is_empty() {
                        let mods_path: LitStr = input.parse()?;
                        let _comma2: Token![,] = input.parse()?;
                        let _bar1c: Token![|] = input.parse()?;
                        let param_ident: Ident = input.parse()?;
                        let _bar2c: Token![|] = input.parse()?;
                        let expr: syn::Expr = input.parse()?;
                        return Ok(Self {
                            kind: SkipIfArgsKind::ModList {
                                mods_path,
                                param_ident,
                                expr,
                            },
                        });
                    }
                }
            }
        }

        // Fallback: a single boolean expression
        let expr: syn::Expr = input.parse()?;
        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected tokens in skip_if(...)",
            ));
        }
        Ok(Self {
            kind: SkipIfArgsKind::Expr(expr),
        })
    }
}

/// Parses the inside of an attribute bracket group.
///
/// Supported shapes:
/// - `skip_if(<bool expr>)`
/// - `skip_if("<mods file>", |Mod| <bool expr>)`
fn parse_skip_if_attribute(stream: TokenStream2) -> syn::Result<Option<SkipIfAttr>> {
    let mut it = stream.into_iter();

    let Some(TokenTree::Ident(id)) = it.next() else {
        return Ok(None);
    };
    if id != "skip_if" {
        return Ok(None);
    }

    let Some(TokenTree::Group(g)) = it.next() else {
        return Err(syn::Error::new(id.span(), "expected skip_if(...)"));
    };
    if g.delimiter() != Delimiter::Parenthesis {
        return Err(syn::Error::new(g.span(), "expected skip_if(...)"));
    }

    // No extra tokens allowed
    if it.next().is_some() {
        return Err(syn::Error::new(
            g.span(),
            "unexpected tokens after skip_if(...)",
        ));
    }

    let args: SkipIfArgs = syn::parse2(g.stream())?;

    Ok(Some(match args.kind {
        SkipIfArgsKind::Expr(expr) => {
            let needs_json = expr_contains_field_access(&expr);
            SkipIfAttr::Expr { expr, needs_json }
        }
        SkipIfArgsKind::ModList {
            mods_path,
            param_ident,
            expr,
        } => {
            let needs_json = expr_contains_field_access(&expr);
            SkipIfAttr::ModList {
                mods_path,
                param_ident,
                expr,
                needs_json,
            }
        }
    }))
}

fn eval_bool_expr_two_idents(
    expr: &syn::Expr,
    root_ident: &Ident,
    root_value: &str,
    param_ident: &Ident,
    param_value: &str,
    json: Option<&serde_json::Value>,
) -> Result<bool, String> {
    use syn::{BinOp, Expr};

    fn eval_value(
        e: &syn::Expr,
        root_ident: &Ident,
        root_value: &str,
        param_ident: &Ident,
        param_value: &str,
        json: Option<&serde_json::Value>,
    ) -> Result<String, String> {
        match e {
            Expr::Paren(p) => eval_value(
                &p.expr,
                root_ident,
                root_value,
                param_ident,
                param_value,
                json,
            ),

            Expr::Lit(lit) => {
                if let syn::Lit::Str(s) = &lit.lit {
                    Ok(s.value())
                } else {
                    Ok(lit.lit.to_token_stream().to_string())
                }
            }

            Expr::Path(p) => {
                if p.path.segments.len() == 1 && p.path.segments[0].ident == *root_ident {
                    Ok(root_value.to_string())
                } else if p.path.segments.len() == 1 && p.path.segments[0].ident == *param_ident {
                    Ok(param_value.to_string())
                } else {
                    Err("unsupported path in skip_if; use the placeholder ident(s)".into())
                }
            }

            Expr::Call(call) => {
                let func = match &*call.func {
                    Expr::Path(p) if p.path.segments.len() == 1 => {
                        p.path.segments[0].ident.to_string()
                    }
                    _ => {
                        return Err(
                            "only simple calls like pascal(x) / snake(x) are supported".into(),
                        )
                    }
                };

                if call.args.len() != 1 {
                    return Err(format!("{func}(...) must have exactly 1 argument"));
                }

                let arg = eval_value(
                    &call.args[0],
                    root_ident,
                    root_value,
                    param_ident,
                    param_value,
                    json,
                )?;
                match func.as_str() {
                    "snake" => Ok(tutorlolv2_fmt::to_ssnake(&arg).to_lowercase()),
                    "pascal" => Ok(tutorlolv2_fmt::pascal_case(&arg)),
                    _ => Err(format!("unknown function in skip_if: {func}")),
                }
            }

            Expr::Field(_) => {
                let json = json.ok_or_else(|| "JSON not loaded".to_string())?;
                let v = resolve_json_path(e, root_ident, json)?;
                json_value_to_string(v)
            }

            _ => Err(
                "unsupported value in skip_if; use Name/Mod, string literal, pascal(...), snake(...), or Name.<json_field>"
                    .into(),
            ),
        }
    }

    fn eval_bool(
        e: &syn::Expr,
        root_ident: &Ident,
        root_value: &str,
        param_ident: &Ident,
        param_value: &str,
        json: Option<&serde_json::Value>,
    ) -> Result<bool, String> {
        match e {
            Expr::Paren(p) => eval_bool(
                &p.expr,
                root_ident,
                root_value,
                param_ident,
                param_value,
                json,
            ),

            Expr::Binary(b) => match &b.op {
                BinOp::And(_) => Ok(eval_bool(
                    &b.left,
                    root_ident,
                    root_value,
                    param_ident,
                    param_value,
                    json,
                )? && eval_bool(
                    &b.right,
                    root_ident,
                    root_value,
                    param_ident,
                    param_value,
                    json,
                )?),
                BinOp::Or(_) => Ok(eval_bool(
                    &b.left,
                    root_ident,
                    root_value,
                    param_ident,
                    param_value,
                    json,
                )? || eval_bool(
                    &b.right,
                    root_ident,
                    root_value,
                    param_ident,
                    param_value,
                    json,
                )?),
                BinOp::Eq(_) => {
                    let l = eval_value(
                        &b.left,
                        root_ident,
                        root_value,
                        param_ident,
                        param_value,
                        json,
                    )?;
                    let r = eval_value(
                        &b.right,
                        root_ident,
                        root_value,
                        param_ident,
                        param_value,
                        json,
                    )?;
                    Ok(l == r)
                }
                BinOp::Ne(_) => {
                    let l = eval_value(
                        &b.left,
                        root_ident,
                        root_value,
                        param_ident,
                        param_value,
                        json,
                    )?;
                    let r = eval_value(
                        &b.right,
                        root_ident,
                        root_value,
                        param_ident,
                        param_value,
                        json,
                    )?;
                    Ok(l != r)
                }
                _ => Err("unsupported operator in skip_if; use ==, !=, &&, ||".into()),
            },

            _ => Err("skip_if condition must be a boolean expression".into()),
        }
    }

    eval_bool(expr, root_ident, root_value, param_ident, param_value, json)
}

fn eval_skip_if_expr(
    expr: &syn::Expr,
    root_ident: &Ident,
    stem: &str,
    json: Option<&serde_json::Value>,
) -> Result<bool, String> {
    let dummy = Ident::new("__unused", Span::call_site());
    eval_bool_expr_two_idents(expr, root_ident, stem, &dummy, "", json)
}

static MODS_CACHE: OnceLock<Mutex<HashMap<PathBuf, (SystemTime, Arc<Vec<String>>)>>> =
    OnceLock::new();

fn load_mods_cached(path: &PathBuf, span: Span) -> syn::Result<Arc<Vec<String>>> {
    let meta = fs::metadata(path)
        .map_err(|e| syn::Error::new(span, format!("failed to stat {}: {e}", path.display())))?;

    let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    let cache = MODS_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    {
        let map = cache.lock().unwrap();
        if let Some((cached_mtime, v)) = map.get(path) {
            if *cached_mtime == mtime {
                return Ok(v.clone());
            }
        }
    }

    let src = fs::read_to_string(path)
        .map_err(|e| syn::Error::new(span, format!("failed to read {}: {e}", path.display())))?;

    let mut mods: Vec<String> = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("pub mod ") {
            let rest = rest.trim();
            // rest expected: <ident>;
            let ident_part = rest.split(';').next().unwrap_or("").trim();
            if !ident_part.is_empty() {
                // strip inline comments
                let ident_part = ident_part.split("//").next().unwrap_or("").trim();
                if !ident_part.is_empty() {
                    mods.push(ident_part.to_string());
                }
            }
        }
    }

    let mods = Arc::new(mods);

    let mut map = cache.lock().unwrap();
    map.insert(path.clone(), (mtime, mods.clone()));

    Ok(mods)
}

enum Node {
    TT(TokenTree),
    Group {
        delim: Delimiter,
        span: Span,
        children: Vec<Node>,
    },
    /// Attribute-like directive: `#[skip_if(...)] <impl ... { ... }>`
    SkipIfImpl {
        span: Span,
        cond: SkipIfAttr,
        item: Vec<Node>,
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
    #[allow(dead_code)]
    root_ident: Ident,
}

fn compile_template(ts: TokenStream2, root_ident: &Ident) -> syn::Result<CompiledTemplate> {
    fn compile_nodes(ts: TokenStream2, root_ident: &Ident) -> syn::Result<Vec<Node>> {
        let mut out = Vec::new();
        let mut it = ts.into_iter().peekable();

        while let Some(tt) = it.next() {
            match tt {
                // Attribute-like directive: #[skip_if(...)] <impl ... { ... }>
                TokenTree::Punct(p) if p.as_char() == '#' => {
                    let Some(TokenTree::Group(attr_g)) = it.peek().cloned() else {
                        out.push(Node::TT(TokenTree::Punct(p)));
                        continue;
                    };
                    if attr_g.delimiter() != Delimiter::Bracket {
                        out.push(Node::TT(TokenTree::Punct(p)));
                        continue;
                    }

                    // Try parse the attribute as `skip_if(...)`
                    if let Some(cond) = parse_skip_if_attribute(attr_g.stream())? {
                        // consume the bracket group
                        let _ = it.next();

                        // Capture the next `impl ... { ... }` item (best-effort).
                        let mut item_ts = TokenStream2::new();
                        let mut saw_impl = false;
                        let mut saw_body = false;

                        while let Some(nxt) = it.next() {
                            match &nxt {
                                TokenTree::Ident(id) if id == "impl" => {
                                    saw_impl = true;
                                }
                                TokenTree::Group(g)
                                    if saw_impl && g.delimiter() == Delimiter::Brace =>
                                {
                                    saw_body = true;
                                }
                                _ => {}
                            }

                            item_ts.extend([nxt]);

                            if saw_body {
                                break;
                            }
                        }

                        if item_ts.is_empty() {
                            return Err(syn::Error::new(
                                p.span(),
                                "#[skip_if(...)] must be followed by an `impl ... { ... }` item",
                            ));
                        }

                        let item_nodes = compile_nodes(item_ts, root_ident)?;
                        out.push(Node::SkipIfImpl {
                            span: p.span(),
                            cond,
                            item: item_nodes,
                        });
                        continue;
                    }

                    // Not our attribute; keep tokens as-is
                    out.push(Node::TT(TokenTree::Punct(p)));
                }

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
        Node::SkipIfImpl { cond, item, .. } => {
            let needs = match cond {
                SkipIfAttr::Expr { needs_json, .. } => *needs_json,
                SkipIfAttr::ModList { needs_json, .. } => *needs_json,
            };
            needs || nodes_need_json(item)
        }
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

            Node::SkipIfImpl { cond, item, .. } => match cond {
                SkipIfAttr::Expr { expr, needs_json } => {
                    if *needs_json && json.is_none() {
                        return Err(syn::Error::new(
                            expr.span(),
                            "skip_if condition requires JSON fields (Name.x), but JSON was not loaded",
                        ));
                    }

                    let skip = eval_skip_if_expr(expr, root_ident, stem, json)
                        .map_err(|msg| syn::Error::new(expr.span(), msg))?;

                    if !skip {
                        let inner = render_nodes(item, root_ident, stem, json)?;
                        out.extend(inner);
                    }
                }

                SkipIfAttr::ModList {
                    mods_path,
                    param_ident,
                    expr,
                    needs_json,
                } => {
                    if *needs_json && json.is_none() {
                        return Err(syn::Error::new(
                            expr.span(),
                            "skip_if(mods, ...) condition requires JSON fields (Name.x), but JSON was not loaded",
                        ));
                    }

                    let mods_path_str =
                        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), mods_path.value());
                    let mods_path_buf = PathBuf::from(mods_path_str);
                    let mods = load_mods_cached(&mods_path_buf, mods_path.span())?;

                    let mut skip = false;
                    for m in mods.iter() {
                        let ok =
                            eval_bool_expr_two_idents(expr, root_ident, stem, param_ident, m, json)
                                .map_err(|msg| syn::Error::new(expr.span(), msg))?;
                        if ok {
                            skip = true;
                            break;
                        }
                    }

                    if !skip {
                        let inner = render_nodes(item, root_ident, stem, json)?;
                        out.extend(inner);
                    }
                }
            },

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
