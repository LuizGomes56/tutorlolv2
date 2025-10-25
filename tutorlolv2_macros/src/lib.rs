use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
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

#[proc_macro_attribute]
pub fn item_generator(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);
    let old_block = func.block;
    func.block = Box::new(syn::parse_quote!({
        #old_block

        let inner = *self;
        let mut this = inner.0;
        Ok(this)
    }));
    TokenStream::from(quote!(#func))
}

fn check_macro_invocation(ts: &TokenStream2) -> Option<proc_macro2::Span> {
    let mut last_ident_is_ability = false;
    let mut last_was_bang = false;

    for tt in ts.clone() {
        match tt {
            TokenTree::Ident(id) => {
                last_ident_is_ability = id == "ability";
                last_was_bang = false;
            }
            TokenTree::Punct(p) => {
                last_was_bang = p.as_char() == '!';
            }
            TokenTree::Group(g) => {
                if last_ident_is_ability && last_was_bang {
                    match g.delimiter() {
                        Delimiter::Bracket => {}
                        Delimiter::Parenthesis | Delimiter::Brace | Delimiter::None => {
                            return Some(g.span());
                        }
                    }
                }
                if let Some(sp) = check_macro_invocation(&g.stream()) {
                    return Some(sp);
                }
                last_ident_is_ability = false;
                last_was_bang = false;
            }
            _ => {
                last_was_bang = false;
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn champion_generator(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    if let Some(sp) = check_macro_invocation(&func.block.to_token_stream()) {
        let err = syn::Error::new(
            sp,
            "macro ability! must be invoked as `ability![...]`. Delimiters `(` or `{` are not supported",
        )
        .to_compile_error();
        return TokenStream::from(quote!(#err));
    }

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let old_block = func.block;

    func.block = Box::new(syn::parse_quote!({
        macro_rules! ability {
            ($field:ident, $idx:literal, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {{
                let pattern = [$(($a, $b, AbilityLike::$field(AbilityName::$c))),*];
                self.extract_ability_damage(AbilityLike::$field(AbilityName::Void), $idx, &pattern);
            }};
            ($field:ident, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {{
                let pattern = [$(($a, $b, AbilityLike::$field(AbilityName::$c))),*];
                self.extract_ability_damage(AbilityLike::$field(AbilityName::Void), 0, &pattern);
            }};
            ($field1:ident::$field2:ident, ($offset1:literal, $offset2:literal)) => {{
                self.extract_passive_damage(AbilityLike::$field1(AbilityName::$field2), ($offset1, $offset2), None, None);
            }};
            ($field1:ident::$field2:ident, ($offset1:literal, $offset2:literal), $postfix:ident) => {{
                self.extract_passive_damage(AbilityLike::$field1(AbilityName::$field2), ($offset1, $offset2), Some(EvalIdent::$postfix), None);
            }};
            ($field1:ident::$field2:ident, ($offset1:literal, $offset2:literal), $scalings:literal) => {{
                self.extract_passive_damage(AbilityLike::$field1(AbilityName::$field2), ($offset1, $offset2), None, Some($scalings));
            }};
            ($field1:ident::$field2:ident, ($offset1:literal, $offset2:literal), $postfix:ident, $scalings:literal) => {{
                self.extract_passive_damage(AbilityLike::$field1(AbilityName::$field2), ($offset1, $offset2), Some(EvalIdent::$postfix), Some($scalings));
            }};
        }

        macro_rules! merge_damage {
            ($closure:expr, $($field1:ident::$field2:ident),+$(,)?) => {{
                let mut sizes = Vec::<usize>::new();
                $(
                    {
                        let key = AbilityLike::$field1(AbilityName::$field2);
                        let arg = self.hashmap
                            .get(&key)
                            .ok_or(format!("Failed to find field: {key:?}"))?;
                        sizes.push(arg.damage.len());
                    }
                )+
                assert!(
                    sizes.windows(2).all(|w| w[0] == w[1]),
                    "Can't compare abilities with different sizes"
                );
                assert!(sizes.len() > 0, "Closure must take at least one argument");
                let mut result = Vec::<String>::with_capacity(sizes[0]);
                for i in 0..sizes[0] {
                    result.push(($closure)(
                        $(get!($field1::$field2).damage[i].clone()),+
                    ));
                }
                result
            }};
        }

        macro_rules! get {
            ($field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                self.hashmap
                    .get(&key)
                    .ok_or(format!("Failed to find field: {key:?}"))?
            }};
        }

        macro_rules! get_mut {
            ($field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                self.hashmap
                    .get_mut(&key)
                    .ok_or(format!("Failed to find field: {key:?}"))?
            }};
        }

        macro_rules! merge {
            ($($f1:ident::$v1:ident <= $f2:ident::$v2:ident),+$(,)?) => {{
                $(
                    self.mergevec.push((
                        AbilityLike::$f1(AbilityName::$v1),
                        AbilityLike::$f2(AbilityName::$v2),
                    ));
                )+
            }};
        }

        macro_rules! clone_to {
            ($field3:ident::$field4:ident => $field1:ident::$field2:ident) => {{
                let clone_from = get!($field3::$field4).clone();
                insert!($field1::$field2, clone_from);
                get_mut!($field1::$field2)
            }};
        }

        macro_rules! insert {
            ($field1:ident::$field2:ident, $value:expr) => {{
                let temp = $value;
                self.hashmap.insert(AbilityLike::$field1(AbilityName::$field2), temp);
            }};
        }

        #old_block;

        if !self
            .mergevec
            .iter()
            .all(|(a, b)| self.hashmap.contains_key(a) && self.hashmap.contains_key(b))
        {
            println!(
                "{} merge vec generated is not consistent. merge vec: {:?}, Keys: {:?}",
                self.data.name,
                self.mergevec,
                self.hashmap.keys().collect::<Vec<_>>()
            );
            return Err("Found inconsistent merge vec".into());
        }

        let inner = *self;
        let mut this = inner.0;

        Ok(this.finish())
    }));

    TokenStream::from(quote!(#func))
}
