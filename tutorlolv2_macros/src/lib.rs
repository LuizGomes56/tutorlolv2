use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input, punctuated::Punctuated, token::Comma};

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
                    let pattern = [$(
                        (
                            $a,
                            $b,
                            AbilityLike::[<$field:upper>](AbilityName::$c),
                        )
                    ),*];
                    extract_ability_damage(&data.abilities.$field[$idx], &mut abilities, &pattern);
                }
            };
            ($field:ident, $(($a:literal, $b:literal, $c:ident)),* $(,)?) => {
                paste::paste! {
                    let pattern = [$((
                            $a,
                            $b,
                            AbilityLike::[<$field:upper>](AbilityName::$c),
                        )
                    ),*];
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

        macro_rules! save_change {
            () => {
                format!("internal/items/{}.json", id).write_to_file(serde_json::to_string_pretty(&cur_value).unwrap().as_bytes())
            };
        };

        macro_rules! cap_parens {
            ($expr:expr, $n:expr) => {{
                let pattern = format!(r"^(?:.*?(\([^()]*\))){{{}}}", $n + 1);
                let re = Regex::new(&pattern)
                    .expect("Falha ao compilar a regex de parênteses");
                re.captures(&$expr)
                    .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                    .expect(&format!("Não existe parênteses #{} em '{}'", $n, $expr))
            }};
        };

        macro_rules! cap_numbers {
            ($expr:expr) => {{
                let re = ::regex::Regex::new(r"\d+")
                    .expect("Falha ao compilar regex de números");
                re.find_iter(&$expr)
                    .map(|m| m.as_str().to_string())
                    .collect::<Vec<String>>()
            }};
        };

        macro_rules! cap_percent {
            ($expr:expr, $n:expr) => {{
                let pattern = format!(r"^(?:.*?(\d+)%){{{}}}", $n + 1);
                let re = Regex::new(&pattern)
                    .expect("Falha ao compilar a regex de percentuais");
                re.captures(&$expr)
                    .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                    .expect(&format!("Não existe percentual #{} em '{}'", $n, $expr))
                    .parse::<f64>().unwrap()
            }};
        };

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
        save_change!()
    }));

    TokenStream::from(quote! {
        #func
    })
}
