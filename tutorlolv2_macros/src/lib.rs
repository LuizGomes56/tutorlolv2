use proc_macro::TokenStream;
use proc_macro2::{Group, Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::{fs, path::PathBuf};
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

    // (stem, full_path)
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

    files.sort_by(|a, b| a.0.cmp(&b.0));

    let (ph_ident, is_array) = match &placeholder {
        Placeholder::Simple(id) => (id.clone(), false),
        Placeholder::Array(id) => (id.clone(), true),
    };

    let tpl = unwrap_outer_group(template);

    let mut pieces: Vec<TokenStream2> = Vec::with_capacity(files.len());

    for (stem, path) in files {
        let id = Ident::new(&stem, Span::call_site());

        // lê JSON
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                return syn::Error::new(
                    dir.span(),
                    format!("failed to read json file {}: {e}", path.display()),
                )
                .to_compile_error()
                .into();
            }
        };

        let json: serde_json::Value = match serde_json::from_slice(&bytes) {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(
                    dir.span(),
                    format!("invalid json in {}: {e}", path.display()),
                )
                .to_compile_error()
                .into();
            }
        };

        // 1) expande diretivas %...% (usando o JSON desse arquivo)
        let expanded = match expand_directives(tpl.clone(), &ph_ident, &stem, &json) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        };

        // 2) substitui o placeholder IDENT fora das diretivas (comportamento antigo)
        let final_ts = subst_ident(expanded, &ph_ident, &id);

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

fn expand_directives(
    ts: TokenStream2,
    root_ident: &Ident,
    stem: &str,
    json: &serde_json::Value,
) -> syn::Result<TokenStream2> {
    let mut out = TokenStream2::new();
    let mut it = ts.into_iter().peekable();

    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Group(g) => {
                let inner = expand_directives(g.stream(), root_ident, stem, json)?;
                let mut ng = Group::new(g.delimiter(), inner);
                ng.set_span(g.span());
                out.extend([TokenTree::Group(ng)]);
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

                let rendered = eval_directive(inner, root_ident, stem, json)?;
                let injected: TokenStream2 = rendered.parse().map_err(|e| {
                    syn::Error::new(
                        start_span,
                        format!(
                            "directive produced invalid Rust tokens: {e} (output: {rendered:?})"
                        ),
                    )
                })?;

                out.extend(injected);
            }

            other => out.extend([other]),
        }
    }

    Ok(out)
}

fn eval_directive(
    inner: TokenStream2,
    root_ident: &Ident,
    stem: &str,
    json: &serde_json::Value,
) -> syn::Result<String> {
    let expr: syn::Expr = syn::parse2(inner)?;
    eval_expr_to_string(&expr, root_ident, stem, json)
        .map_err(|msg| syn::Error::new(expr.span(), msg))
}

fn eval_expr_to_string(
    expr: &syn::Expr,
    root_ident: &Ident,
    stem: &str,
    json: &serde_json::Value,
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

        // File.x.y  (acesso JSON)
        Expr::Field(_) => {
            let v = resolve_json_path(expr, root_ident, stem, json)?;
            Ok(json_value_to_string(v)?)
        }

        // File (sozinho) -> retorna o stem do arquivo (útil em alguns casos)
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

/// Resolve File.a.b.c no JSON do arquivo.
/// Regras:
/// - a raiz precisa ser exatamente o `root_ident` (ex.: File)
/// - cada `.campo` vira uma chave no JSON
fn resolve_json_path<'a>(
    expr: &syn::Expr,
    root_ident: &Ident,
    _stem: &str,
    json: &'a serde_json::Value,
) -> Result<&'a serde_json::Value, String> {
    // Extrai a cadeia de campos: File.name.x...
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

/// Converte o valor para String.
/// Se for objeto e existir a chave "Value", usa ela.
/// - String -> sem aspas
/// - Number/Bool -> to_string()
/// - Object/Array -> JSON compact (to_string()) (não ideal p/ ident)
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
