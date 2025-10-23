use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::fs;
use syn::{
    Ident, LitStr, Token, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma,
};

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
pub fn item_gen_v2(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);
    func.block = Box::new(syn::parse_quote!({ todo!() }));
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
pub fn generator_v2(_args: TokenStream, input: TokenStream) -> TokenStream {
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

/// ## Provides useful macros to extract data from CDN API
///
/// Macros: `ability!`, `passive!`, `get!`, `insert!`, `merge_damage!`
///
/// - Hover over each macro for more information
#[proc_macro_attribute]
pub fn generator(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let map_decl = quote! {
        let mut abilities = HashMap::<AbilityLike, Ability>::new();
        let mut merge_data = Vec::new();

        macro_rules! ability {
            ($field:ident, $idx:literal, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {
                paste::paste! {
                    let pattern = [$(($a, $b, AbilityLike::[<$field:upper>](AbilityName::$c))),*];
                    extract_ability_damage(&data.abilities.$field[$idx], &mut abilities, &pattern);
                }
            };
            ($field:ident, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {
                paste::paste! {
                    let pattern = [$(($a, $b, AbilityLike::[<$field:upper>](AbilityName::$c))),*];
                    extract_ability_damage(&data.abilities.$field[0], &mut abilities, &pattern);
                }
            };
        }

        /// ### `passive!`: fn `extract_passive_damage` in a more convenient way
        ///
        /// If more than 3 arguments are supplied, a tuple with `(Scalling, Postfix)` is expected
        /// Otherwise, just the name of the key, its coordinates and the target vector are expected
        ///
        /// ```
        /// passive!("P", (0, 0), MINIMUM);
        /// passive!("P", (0, 0), MINIMUM, (None, Some("ENEMY_MAX_HEALTH")));
        /// passive!("P", (0, 0), MAXIMUM, (Some(<usize>scalling), Some("POSTFIX")));
        /// ```
        macro_rules! passive {
            ($key:ident, ($i:literal, $j:literal)) => {{
                extract_passive_damage(
                    &data,
                    ($i, $j),
                    None,
                    None,
                    AbilityLike::P(AbilityName::$key),
                    &mut abilities
                );
            }};
            ($key:ident, ($i:literal, $j:literal), ($scaling:expr, $postfix:expr)) => {{
                extract_passive_damage(
                    &data,
                    ($i, $j),
                    $postfix,
                    $scaling,
                    AbilityLike::P(AbilityName::$key),
                    &mut abilities,
                );
            }};
        }

        macro_rules! merge_damage {
            ($size:literal, $closure:expr, $($field1:ident::$field2:ident),*$(,)?) => {{
                let mut result = Vec::<String>::with_capacity($size);
                for i in 0..$size {
                    let args = ($(
                        abilities.get(
                            &AbilityLike::$field1(
                                AbilityName::$field2
                            )
                        )
                            .unwrap()
                            .damage[i]
                            .clone(),
                        )+);
                    result.push($closure(args));
                }
                result
            }};
        }

        macro_rules! get {
            ($field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                match abilities.get(&key) {
                    Some(value) => value,
                    None => {
                        panic!("macro [get!]: Error: key does not exist: {:?}", key);
                    }
                }
            }};
            (mut $field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                match abilities.get_mut(&key) {
                    Some(value) => value,
                    None => {
                        panic!("macro [get!]: Error: key does not exist: {:?}", key);
                    }
                }
            }};
        }

        /// ### `insert!`
        ///
        /// It inserts the value `{VALUE}` into the `HashMap` under the key `{STRING}`
        ///
        /// ```
        /// insert!("Q", Ability {...});
        /// ```
        macro_rules! insert {
            ($field1:ident::$field2:ident, $value:expr) => {{
                abilities.insert(AbilityLike::$field1(AbilityName::$field2), $value);
            }};
        }

        macro_rules! clone_to {
            ($field3:ident::$field4:ident => $field1:ident::$field2:ident) => {{
                abilities.insert(
                    AbilityLike::$field1(AbilityName::$field2),
                    get!($field3::$field4).clone(),
                );
                get!(mut $field1::$field2)
            }};
        }

        macro_rules! merge_with {
            ($(($f1:ident::$v1:ident, $f2:ident::$v2:ident)),+$(,)?) => {{
                $(
                    merge_data.push((
                        AbilityLike::$f1(AbilityName::$v1),
                        AbilityLike::$f2(AbilityName::$v2),
                    ));
                )+
            }};
        }
    };

    let old_block = func.block;
    func.block = Box::new(syn::parse_quote!({
        #map_decl
        #old_block;
        data.format(abilities, merge_data)
    }));

    TokenStream::from(quote! {
        #func
    })
}

/// Attribute to trace function entry and execution time
#[proc_macro_attribute]
pub fn trace_time(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as syn::ItemFn);
    let fn_name = function.sig.ident.to_string();
    let original_block = function.block;
    let is_async = function.sig.asyncness.is_some();
    let output = &function.sig.output;
    let timed_block = if is_async {
        quote! {{
            println!("fn[{}]", #fn_name);
            let __start = std::time::Instant::now();
            let __result = (async move #original_block).await;
            let __elapsed = __start.elapsed();
            println!("fn[{}] took {:?}", #fn_name, __elapsed);
            __result
        }}
    } else {
        quote! {{
            println!("fn[{}]", #fn_name);
            let __start = std::time::Instant::now();
            let __result = (|| #output {
                #original_block
            })();
            let __elapsed = __start.elapsed();
            println!("fn[{}] took {:?}", #fn_name, __elapsed);
            __result
        }}
    };
    function.block = Box::new(syn::parse2(timed_block).expect("Failed to parse timed block"));
    TokenStream::from(quote! { #function })
}

#[proc_macro_attribute]
pub fn item_generator(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    let args = parse_macro_input!(args with Punctuated::<Ident, Comma>::parse_terminated);
    let dt_ident = args.iter().nth(0).cloned();
    let attrs_ident = args.iter().nth(1).cloned();

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let set_damage_type = if let Some(variant) = dt_ident {
        quote! {
            cur_value.damage_type = tutorlolv2_gen::DamageType::#variant;
        }
    } else {
        quote! {
            cur_value.damage_type = tutorlolv2_gen::DamageType::default();
        }
    };

    let set_attrs = if let Some(variant) = attrs_ident {
        quote! {
            cur_value.attributes = tutorlolv2_gen::Attrs::#variant;
        }
    } else {
        quote! {}
    };

    let ident = &func.sig.ident;

    let expand_decl = quote! {
        let id = stringify!(#ident)
            .split("_")
            .last()
            .unwrap();

        let cdn_value = format!("cache/cdn/items/{}.json", id).read_json::<CdnItem>()?;
        let mut cur_value = format!("internal/items/{}.json", id).read_json::<Item>()?;

        macro_rules! write_dmg {
            (@ranged $ranged:expr, @melee $melee:expr) => {{
                cur_value.ranged = Some(DamageObject {
                    minimum_damage: Some($ranged.clone()),
                    maximum_damage: None,
                });
                cur_value.melee = Some(DamageObject {
                    minimum_damage: Some($melee),
                    maximum_damage: None,
                });
            }};
            ($min_dmg:expr) => {{
                cur_value.ranged = Some(DamageObject {
                    minimum_damage: Some($min_dmg.clone()),
                    maximum_damage: None,
                });
                cur_value.melee = Some(DamageObject {
                    minimum_damage: Some($min_dmg),
                    maximum_damage: None,
                });
            }};
            ($min_dmg:expr, $max_dmg:expr) => {{
                cur_value.ranged = Some(DamageObject {
                    minimum_damage: Some($min_dmg.clone()),
                    maximum_damage: Some($max_dmg.clone()),
                });
                cur_value.melee = Some(DamageObject {
                    minimum_damage: Some($min_dmg),
                    maximum_damage: Some($max_dmg),
                });
            }};
        };

        #set_damage_type
        #set_attrs
    };

    let old_block = func.block;
    func.block = Box::new(syn::parse_quote!({
        #expand_decl
        #old_block
        format!("internal/items/{}.json", id).write_to_file(serde_json::to_string_pretty(&cur_value).unwrap().as_bytes())
    }));

    TokenStream::from(quote! {
        #func
    })
}
