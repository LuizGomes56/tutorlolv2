use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input, punctuated::Punctuated, token::Comma};

/// ## Provides useful macros to extract data from CDN API
///
/// Macros: `ability!`, `passive!`, `get!`, `insert!`, `merge_ability!`, `merge_damage!`
///
/// - Hover over each macro for more information
#[proc_macro_attribute]
pub fn generator(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let map_decl = quote! {
        let mut abilities = HashMap::<String, Ability>::new();

        /// ### `ability!`: fn `extract_ability_damage` in a more convenient way
        ///
        /// It can be used in two different ways. The most common one is:
        ///
        /// ```
        /// ability!(q, (0, 0, "Q_MAX", MAXIMUM));
        /// ability!(r, (0, 4, "R", MINIMUM));
        /// ```
        ///
        /// Notice that it defaults that the ability being requested is unique.
        /// It means that it replaces `arg1` with: `&data.abilities.$field[0]`.
        /// Not recommended to use the first pattern for champions that have
        /// two different effects in the same ability such as ***Hwei, Gnar, Elise, Nidalee, ...***
        ///
        /// Instead, you wanna use the second pattern:
        ///
        /// ```
        /// ability!(q, 1, (0, 0, "Q_MAX", MAXIMUM));
        /// ability!(r, 2, (0, 4, "R", MINIMUM));
        /// ```
        macro_rules! ability {
            ($field:ident, $idx:literal, $(($a:literal, $b:literal, $c:ident, $d:ident)),* $(,)?) => {
                paste::paste! {
                    let pattern = [$(
                        (
                            $a,
                            $b,
                            AbilityLike::[<$field:upper>](AbilityName::$c).[<to_str_ $field>](),
                            Target::$d
                        )
                    ),*];
                    extract_ability_damage(&data.abilities.$field[$idx], &mut abilities, &pattern);
                }
            };
            ($field:ident, $(($a:literal, $b:literal, $c:ident, $d:ident)),* $(,)?) => {
                paste::paste! {
                    let pattern = [$((
                            $a,
                            $b,
                            AbilityLike::[<$field:upper>](AbilityName::$c).[<to_str_ $field>](),
                            Target::$d
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
            ($key:ident, ($i:literal, $j:literal), $target:expr) => {
                paste::paste! {
                    extract_passive_damage(
                        &data,
                        ($i, $j),
                        None,
                        None,
                        &Target::$target,
                        AbilityLike::P(AbilityName::$key).to_str_p(),
                        &mut abilities
                    );
                }
            };
            ($key:ident, ($i:literal, $j:literal), $target:expr, ($scaling:expr, $postfix:expr)) => {
                paste::paste! {
                    extract_passive_damage(
                        &data,
                        ($i, $j),
                        $postfix,
                        $scaling,
                        &Target::$target,
                        AbilityLike::P(AbilityName::$key).to_str_p(),
                        &mut abilities,
                    );
                }
            };
        }

        /// ### `merge_ability!`
        ///
        /// Has two variants. The one with only one argument takes the `{STRING}` and checks for
        /// the key `{STRING}_MAX` in the `HashMap` and if found, moves the value from `"maximum_damage"` of it
        /// into the original key, that should represent the `minimum_damage`. At the end, the key `{STRING}_MAX`
        /// is removed from the `HashMap`.
        ///
        /// The other variant takes two arguments, the first one is the key that represent `minimum_damage`,
        /// and the second one is the key that represent `maximum_damage`.
        ///
        /// ```
        /// merge_ability!("Q");
        /// merge_ability!("Q_MIN", "Q_MAX");
        /// ```
        macro_rules! merge_ability {
            ($into_f:ident::$into:ident, $from_f:ident::$from:ident) => {
                paste::paste! {
                    let from_str = AbilityLike::$from_f(AbilityName::$from).[<to_str_ $from_f:lower>]();
                    let into_str = AbilityLike::$into_f(AbilityName::$into).[<to_str_ $into_f:lower>]();
                    let max_dmg = abilities.get(from_str);
                    if let Some(value) = max_dmg {
                        let max_damage = value.maximum_damage.clone();
                        if let Some(mut_ref) = abilities.get_mut(into_str) {
                            mut_ref.maximum_damage = max_damage;
                            abilities.remove(from_str);
                        } else {
                            panic!("macro [merge_ability!]: Error: Destination key from arg#2 does not exist: {:?}", into_str);
                        }
                    } else {
                        panic!("macro [merge_ability!]: Error: key from arg#1 does not exist: {:?}", from_str);
                    }
                }
            };
        }

        /// ### `merge_damage!`
        ///
        /// Has two similar variants. Returns a `Vec<String>` with length = `$size` that is generated by
        /// the closure passed as second argument. This closure can take infinite arguments, and
        /// the value it returns will be inserted into the Vec. If key **R** is included, then the first
        /// argument most likely has to be 3 or 4. Any number greater than that will cause a panic.
        ///
        ///
        /// ```
        /// let q_max = merge_damage!(
        ///     5,
        ///     || format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        ///     (q, minimum_damage)
        /// );
        /// let variant_1 = merge_damage!(
        ///     3,
        ///     || format!("({}) + ({}) + ({}) + ({})", q, w, e, r),
        ///     (q, minimum_damage)
        ///     (w, minimum_damage)
        ///     (e, maximum_damage)
        ///     (r, maximum_damage)
        /// );
        /// ```
        ///
        /// Note that `"q"` is a variable created in the tuple and that can be used inside the closure.
        /// this variable will be uppercased and in each iteration `"q"` will be the same as
        /// ```
        /// abilities.get("Q").unwrap().minimum_damage[i].clone()
        /// ```
        ///
        /// Variant number two is similar but the usage is:
        ///
        /// ```
        /// let q_max = merge_damage!(
        ///     5,
        ///     |(q,)| format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        ///     ("Q", minimum_damage)
        /// );
        /// let variant_2 = merge_damage!(
        ///     3,
        ///     |(q, w, e, r)| format!("({}) + ({}) + ({}) + ({})", q, w, e, r),
        ///     ("Q", minimum_damage)
        ///     ("W", minimum_damage)
        ///     ("E", maximum_damage)
        ///     ("R", maximum_damage)
        /// );
        /// ```
        ///
        /// Variables are created inside the closure instead of outside it.
        macro_rules! merge_damage {
            ($size:literal, $closure:expr, $(($field1:ident::$field2:ident, $field:ident)),+ $(,)?) => {
                paste::paste! {{
                    let mut result = Vec::<String>::with_capacity($size);
                    for i in 0..$size {
                        let args = ($(
                            abilities.get((
                                AbilityLike::[<$field1>](
                                    AbilityName::$field2
                                )
                                .[<to_str_ $field1:lower>]()
                            ))
                                .unwrap()
                                .$field[i]
                                .clone(),
                            )+);
                        result.push($closure(args));
                    }
                    result
                }}
            };
        }

        /// ### `get!`
        ///
        /// It returns a (mutable or not) reference to the key `{STRING}` in the `HashMap`. Panics if not found
        ///
        /// ```
        /// get!("Q"); // ref
        /// get!(mut "Q_MIN"); // mut ref
        /// ```
        macro_rules! get {
            ($field1:ident::$field2:ident) => {
                paste::paste! {{
                    let key = AbilityLike::[<$field1>](AbilityName::$field2).[<to_str_ $field1:lower>]();
                    match abilities.get(key) {
                        Some(value) => value,
                        None => {
                            panic!("macro [get!]: Error: key does not exist: {}", key);
                        }
                    }
                }}
            };
            (mut $field1:ident::$field2:ident) => {
                paste::paste! {{
                    let key = AbilityLike::[<$field1>](AbilityName::$field2).[<to_str_ $field1:lower>]();
                    match abilities.get_mut(key) {
                        Some(value) => value,
                        None => {
                            panic!("macro [get!]: Error: key does not exist: {}", key);
                        }
                    }
                }}
            };
        }

        /// ### `insert!`
        ///
        /// It inserts the value `{VALUE}` into the `HashMap` under the key `{STRING}`
        ///
        /// ```
        /// insert!("Q", Ability {...});
        /// ```
        macro_rules! insert {
            ($field1:ident::$field2:ident, $value:expr) => {
                paste::paste! {{
                    let key = AbilityLike::[<$field1>](AbilityName::$field2).[<to_str_ $field1:lower>]();
                    abilities.insert(key.to_string(), $value);
                }}
            };
        }
    };

    let old_block = func.block;
    func.block = Box::new(syn::parse_quote!({
        #map_decl
        #old_block
        data.format(abilities)
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
            cur_value.damage_type = Some(
                tutorlolv2_generated::DamageType::#variant.to_string()
            );
        }
    } else {
        quote! {
            cur_value.damage_type = Some(
                tutorlolv2_generated::DamageType::default().to_string()
            );
        }
    };

    let set_attrs = if let Some(variant) = attrs_ident {
        quote! {
            cur_value.attributes = tutorlolv2_generated::Attrs::#variant;
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

        let cdn_value = read_json_file::<CdnItem>(&format!("cache/cdn/items/{}.json", id))?;
        let mut cur_value = read_json_file::<Item>(&format!("internal/items/{}.json", id))?;

        macro_rules! save_change {
            () => {{
                let path = format!("internal/items/{}.json", id);
                let json = serde_json::to_string_pretty(&cur_value).unwrap();
                write_to_file(&path, json.as_bytes())
            }};
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
