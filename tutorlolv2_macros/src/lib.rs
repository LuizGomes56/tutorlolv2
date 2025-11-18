use proc_macro::TokenStream;
use proc_macro2::{Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::fs;
use syn::{Ident, LitStr, Token, parse::Parse, parse_macro_input};

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

    let mut stems = Vec::new();
    for ent in entries.flatten() {
        let path = ent.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("json") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                stems.push(stem.to_string());
            }
        }
    }
    if stems.is_empty() {
        return syn::Error::new(dir.span(), "no .json files found in directory")
            .to_compile_error()
            .into();
    }
    stems.sort();

    let (ph_ident, is_array) = match &placeholder {
        Placeholder::Simple(id) => (id, false),
        Placeholder::Array(id) => (id, true),
    };

    let tpl = unwrap_outer_group(template);

    let pieces = stems
        .iter()
        .map(|name| {
            let id = Ident::new(name, Span::call_site());
            subst_ident(tpl.clone(), ph_ident, &id)
        })
        .collect::<Vec<_>>();

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

fn subst_ident(ts: TokenStream2, from: &Ident, to: &Ident) -> TokenStream2 {
    ts.into_iter()
        .map(|tt| match tt {
            TokenTree::Ident(id) if id == *from => TokenTree::Ident(to.clone()),
            TokenTree::Group(g) => {
                let inner = subst_ident(g.stream(), from, to);
                let mut ng = Group::new(g.delimiter(), inner);
                ng.set_span(g.span());
                TokenTree::Group(ng).into()
            }
            other => other.into(),
        })
        .collect()
}
