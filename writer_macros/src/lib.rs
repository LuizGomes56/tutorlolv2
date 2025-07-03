use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
/// ## Provides useful macros to extract data from CDN API
///
/// Macros: `ability!`, `passive!`, `get!`, `insert!`, `merge_ability!`, `merge_damage!`
///
/// - Hover over each macro for more information
#[proc_macro_attribute]
pub fn writer(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let map_decl = quote! {
        let mut abilities = FxHashMap::<String, Ability>::default();

        /// ### `ability!`: fn `extract_ability_damage` in a more convenient way
        ///
        /// It can be used in two different ways. The most common one is:
        ///
        /// ```
        /// ability!(q, (0, 0, "Q_MAX", Target::MAXIMUM));
        /// ability!(r, (0, 4, "R", Target::MINIMUM));
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
        /// ability!(q, 1, (0, 0, "Q_MAX", Target::MAXIMUM));
        /// ability!(r, 2, (0, 4, "R", Target::MINIMUM));
        /// ```
        macro_rules! ability {
            ($field:ident, $idx:literal, $(($a:literal, $b:literal, $c:literal, $d:expr)),* $(,)?) => {
                let pattern = [$(($a, $b, $c, $d)),*];
                extract_ability_damage(&data.abilities.$field[$idx], &mut abilities, &pattern);
            };
            ($field:ident, $(($a:literal, $b:literal, $c:literal, $d:expr)),* $(,)?) => {
                let pattern = [$(($a, $b, $c, $d)),*];
                extract_ability_damage(&data.abilities.$field[0], &mut abilities, &pattern);
            };
        }

        /// ### `passive!`: fn `extract_passive_damage` in a more convenient way
        ///
        /// If more than 3 arguments are supplied, a tuple with `(Scalling, Postfix)` is expected
        /// Otherwise, just the name of the key, its coordinates and the target vector are expected
        ///
        /// ```
        /// passive!("P", (0, 0), Target::MINIMUM);
        /// passive!("P", (0, 0), Target::MINIMUM, (None, Some("ENEMY_MAX_HEALTH")));
        /// passive!("P", (0, 0), Target::MAXIMUM, (Some(<usize>scalling), Some("POSTFIX")));
        /// ```
        macro_rules! passive {
            ($key:literal, ($i:literal, $j:literal), $target:expr) => {
                extract_passive_damage(&data, ($i, $j), None, None, &$target, $key, &mut abilities);
            };
            ($key:literal, ($i:literal, $j:literal), $target:expr, ($scaling:expr, $postfix:expr)) => {
                extract_passive_damage(
                    &data,
                    ($i, $j),
                    $postfix,
                    $scaling,
                    &$target,
                    $key,
                    &mut abilities,
                );
            };
        }

        /// ### `merge_ability!`
        ///
        /// Has two variants. The one with only one argument takes the `{STRING}` and checks for
        /// the key `{STRING}_MAX` in the `FxHashMap` and if found, moves the value from `"maximum_damage"` of it
        /// into the original key, that should represent the `minimum_damage`. At the end, the key `{STRING}_MAX`
        /// is removed from the `FxHashMap`.
        ///
        /// The other variant takes two arguments, the first one is the key that represent `minimum_damage`,
        /// and the second one is the key that represent `maximum_damage`.
        ///
        /// ```
        /// merge_ability!("Q");
        /// merge_ability!("Q_MIN", "Q_MAX");
        /// ```
        macro_rules! merge_ability {
            ($key:literal) => {
                let max_dmg = abilities.get(&format!("{}_MAX", $key));
                if let Some(value) = max_dmg {
                    let max_damage = value.maximum_damage.clone();
                    if let Some(mut_ref) = abilities.get_mut($key) {
                        mut_ref.maximum_damage = max_damage;
                        abilities.remove(&format!("{}_MAX", $key));
                    } else {
                        panic!("macro [merge_ability!]: Error: Destination key from arg#2 does not exist: {:?}", $key);
                    }
                } else {
                    panic!("macro [merge_ability!]: Error: key from arg#1 does not exist: {:?}_MAX", $key);
                }
            };
            ($into:literal, $from:literal) => {
                let max_dmg = abilities.get(&format!($from));
                if let Some(value) = max_dmg {
                    let max_damage = value.maximum_damage.clone();
                    if let Some(mut_ref) = abilities.get_mut($into) {
                        mut_ref.maximum_damage = max_damage;
                        abilities.remove(&format!($from));
                    } else {
                        panic!("macro [merge_ability!]: Error: Destination key from arg#2 does not exist: {:?}", $into);
                    }
                } else {
                    panic!("macro [merge_ability!]: Error: key from arg#1 does not exist: {:?}", $from);
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
            ($size:literal, $closure:expr, $(($key:ident, $field:ident)),+ $(,)?) => {{
                let mut result = Vec::<String>::with_capacity($size);
                for i in 0..$size {
                    $(
                        let $key = abilities.get(stringify!($key).to_uppercase().as_str()).unwrap().$field[i].clone();
                    )+
                    let processed = $closure();
                    result.push(processed);
                }
                result
            }};
            ($size:literal, $closure:expr, $(($key:expr, $field:ident)),+ $(,)?) => {{
                let mut result = Vec::<String>::with_capacity($size);
                for i in 0..$size {
                    let args = ($(abilities.get($key).unwrap().$field[i].clone(),)+);
                    result.push($closure(args));
                }
                result
            }};
        }

        /// ### `get!`
        ///
        /// It returns a (mutable or not) reference to the key `{STRING}` in the `FxHashMap`. Panics if not found
        ///
        /// ```
        /// get!("Q"); // ref
        /// get!(mut "Q_MIN"); // mut ref
        /// ```
        macro_rules! get {
            ($key:literal) => {
                match abilities.get($key) {
                    Some(value) => value,
                    None => {
                        panic!("macro [get!]: Error: key does not exist: {}", $key);
                    }
                }
            };
            (mut $key:literal) => {
                match abilities.get_mut($key) {
                    Some(value) => value,
                    None => {
                        panic!("macro [get!]: Error: key does not exist: {}", $key);
                    }
                }
            };
        }

        /// ### `insert!`
        ///
        /// It inserts the value `{VALUE}` into the `FxHashMap` under the key `{STRING}`
        ///
        /// ```
        /// insert!("Q", Ability {...});
        /// ```
        macro_rules! insert {
            ($key:literal, $value:expr) => {
                abilities.insert($key.to_uppercase(), $value);
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
pub fn item_generator(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let ident = &func.sig.ident;

    let expand_decl = quote! {
        let id = stringify!(#ident)
            .split("_")
            .last()
            .unwrap();

        let cdn_value = read_json_file::<CdnItem>(&format!("cache/cdn/items/{}.json", id))?;
        let mut cur_value = read_json_file::<Item>(&format!("internal/items/{}.json", id))?;

        macro_rules! save_change {
            ($item:expr) => {{
                let path = format!("internal/items/{}.json", id);
                let json = serde_json::to_string_pretty(&$item).unwrap();
                write_to_file(&path, json.as_bytes())
            }};
        };

        macro_rules! write_dmg {
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
    };

    let old_block = func.block;
    func.block = Box::new(syn::parse_quote!({
        #expand_decl
        #old_block
    }));

    TokenStream::from(quote! {
        #func
    })
}
