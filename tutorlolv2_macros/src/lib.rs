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

    func.attrs.push(syn::parse_quote! {
        #[allow(unused_macros)]
    });

    let old_block = func.block;

    func.block = Box::new(syn::parse_quote!({
        #old_block;

        Ok(self.0)
    }));
    TokenStream::from(quote!(#func))
}

/// Verifies if the macro `ability!` was called with the `[` and `]` delimiters. If it
/// was not, returns the region where an unexpected delimiter was called
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

/// Provides useful macros to manipulate the `meraki` json files and create a new object
/// that will be serialized to a json, which is an intermediary representation for the
/// `tutorlolv2_build` script, that will transform them into Rust code, that will be then
/// compiled to machine code, removing any overhead for those operations since they're known
/// at compile time.
///
/// ### Provided macros:
/// `ability`, `merge_damage`, `get`, `attr`, `damage_type`, `get_mut`, `merge`, `clone_to`, `insert`
///
/// Each one already has its own documentation. Hover over it to check their details.
///
/// Note: None of these macros are magic, they're mostly syntax sugar to avoid writing the full enum
/// definition for Abilities, which is `AbilityLike::$(AbilityName::$)`. Most of them simply call
/// `self.$function` directly, positioning the arguments correctly. There are no proc-macros involved.
/// Macro `ability!` is forced to be called with `[` and `]` delimiters so the offset checker can detect
/// easily those bounds and verify if they have changed or not.
///
/// Automatically adds minimum and maximum damages to the `self.mergevec`, checks if they're consistent,
/// and that all keys provided in the mergevec exist in the `self.hashmap` definition, and emits a warning
/// if an ability has unknown damage type. Finally, makes the function return automatically, so the function
/// definition returns an empty type, but when the macro is expanded, the correct return type is assigned.
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
        /// Calls the `self.extract_ability_damage`, and `self.extract_passive_damage` for the given abilities.
        /// This macro can be used in several different ways, and must be invoked as `ability![...]`, with
        /// square brackets as delimiters, otherwise compilation will fail. The first argument is always a char
        /// that can be: `P`, `Q`, `W`, `E`, `R`, and the next arguments are tuples with 3 elements:
        /// `(a, b, c)`, where `a` is the index within the array `effects`, `b` is the index within the array
        /// `levelings`, and `c` is the name of the ability that will be assigned to the hashmap.
        /// If the macro receives a number as second argument, it means that you're accessing the ability array
        /// at the given `index`, not just the first ability. This index in particular should be used only with
        /// champions that transform, such as `Elise`, `Gnar`, `Jayce`, ...
        ///
        /// ### Examples of usage for regular abilities:
        /// ```rs
        /// ability![Q, (0, 0, Min), (0, 1, Max)];
        ///
        /// // Instead of searching at ability in index zero, it does for index 1
        /// ability![W, 1, (0, 1, _1Min), (0, 2, _1Max)];
        /// ```
        ///
        /// It means that the structure in the `meraki` file is something like:
        /// ```jsonc
        /// {
        ///     "abilities": [
        ///         {
        ///             "effects": [
        ///                 {
        ///                     "levelings": [
        ///                         // The next two objects are accessed and their damages are recovered,
        ///                         // based on the second argument of the tuple provided (0 and 1)
        ///                         { ... },
        ///                         { ... },
        ///                         // Any other unaccessed levelings
        ///                         { ... }
        ///                     ]
        ///                 },
        ///                 // Both tuples have first argument `0`, so this effect is ignored
        ///                 { ... }
        ///             ]
        ///         },
        ///         // It is unreachable, since the second argument is a tuple, not an index.
        ///         // meaning that only the first ability is taken
        ///         { ... }
        ///     ]
        /// }
        /// ```
        ///
        /// At the end, the following keys will be added to `self.hashmap`:
        /// `AbilityLike::Q(AbilityName::Min)` and `AbilityLike::Q(AbilityName::Max)`.
        ///
        /// Note that they're `Min` and `Max` pair, so they will be automatically added to `self.mergevec`
        /// at the end of the execution of the function, so they will be displayed in the frontend application
        /// in the format `Min` - `Max`
        ///
        /// ### Extracting passive damage
        ///
        /// Examples of usage:
        ///
        /// ```rs
        /// ability![P::Void, (0, 0)];
        /// ability![P::Min, (0, 0), 2];
        /// ability![P::Max, (0, 0), EnemyHealth];
        /// ability![P::Monster, (0, 0), EnemyHealth, 2];
        /// ```
        ///
        /// Passives similar ways of calling, byt they have two extra arguments:
        /// `scalings` refer to where the objects like `(+ 80% AP)` or `(+ 20% current AD)` are located in the
        /// description fields, in the `effect` offset. If this argument is not provided, the function will
        /// try to find them in the offsets provided in the tuple, which in many cases will work.
        ///
        /// `postfix` multiplies an `EvalIdent` at the end of any expression captured by the `extract_passive_damage`
        /// function. So if it is found that the passive text is: `10 : 180 + (+ 80% AP)` for example, the final
        /// result with Postfix `EvalIdent::EnemyHealth` will be:
        /// ```json
        /// [
        ///     "(10 + (0.8 * ctx.ap)) * ctx.enemy_health",
        ///     // There are 18 levels, and X : Y is a range of them, so from 10 to 180 means that
        ///     // every level, the base damage increases by 10, starting at 10 and ending at 180
        ///     "(20 + (0.8 * ctx.ap)) * ctx.enemy_health",
        ///     "(30 + (0.8 * ctx.ap)) * ctx.enemy_health",
        ///     ...
        ///     // Damage at level 18. If in execution the level is > 18 as in URF mode, it will
        ///     // default the damage to zero
        ///     "(180 + (0.8 * ctx.ap)) * ctx.enemy_health",
        /// ]
        /// ```
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

        /// Takes in a closure and any number of arguments that will be passed to that given closure.
        /// During the execution of the closure, all fields passed as arguments should be present in
        /// `self.hashmap`, otherwise the function will fail. Also, all arguments should have the same
        /// length for their `damage` as values in the hashmap. For example:
        /// ```ts
        /// self.hashmap = {
        ///     "Q::_1": {
        ///         "damage": [10, 20, 30]
        ///     },
        ///     "Q::_2": {
        ///         "damage": [50, 60, 80]
        ///     }
        /// }
        /// ```
        ///
        /// Through the example, we can do:
        /// ```rs
        /// let damage = merge_damage!(
        ///     |q1, q2| format!("{q1} * {q2}"),
        ///     Q::_1,
        ///     Q::_2,
        /// );
        /// insert!(
        ///     Q::Max,
        ///     Ability {
        ///         damage,
        ///         ..
        ///     }
        /// )
        /// ```
        ///
        /// The result will be:
        /// ```ts
        /// self.hashmap = {
        ///     "Q::_1": {
        ///         "damage": [10, 20, 30]
        ///     },
        ///     "Q::_2": {
        ///         "damage": [50, 60, 80]
        ///     },
        ///     "Q::Max": {
        ///         "damage": ["10 * 50", "20 * 60", "30 * 80"]
        ///     }
        /// }
        /// ```
        ///
        /// Note that arguments are defined by position, not by name, and that the
        /// closure should always return a string, which will be the assigned damage at the end.
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

        /// Gets an item from the `self.hashmap` object, and fails if it does not exist.
        /// Example:
        /// ```rs
        /// let q_void: &Ability = get![Q::Void];
        /// ```
        macro_rules! get {
            ($field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                self.hashmap
                    .get(&key)
                    .ok_or(format!("Failed to find field: {key:?}"))?
            }};
        }

        /// Assigns an attribute to a single key in the `self.hashmap`, or a group of them.
        /// Example:
        /// ```rs
        /// attr![
        ///     AreaOnhit => [Q::Min, E::Max, R::Void, W::_1],
        ///     Area => [Q::Max],
        ///     // Not all possibilities need to be covered, only those that will be assigned
        ///     Onhit => [W::_2, R::Max],
        ///     // Unnecessary. Everything has `None` by default
        ///     None => [],
        /// ];
        /// ```
        macro_rules! attr {
            ($($attr:ident => [$($field1:ident::$field2:ident),+$(,)?]),+$(,)?) => {
                {
                    $(
                        $(
                            get_mut![$field1::$field2].attributes = Attrs::$attr;
                        )+
                    )+
                }
            };
        }

        /// Assigns a damage type for some expression already in the `self.hashmap` object.
        /// Example:
        /// ```rs
        /// damage_type![R::Max, Adaptative];
        /// damage_type![W::_3Max, Magic];
        /// damage_type![E::_1, Physical];
        /// ```
        macro_rules! damage_type {
            ($field1:ident::$field2:ident, $damage_type:ident) => {{
                get_mut![$field1::$field2].damage_type = DamageType::$damage_type;
            }}
        }

        /// Gets a mutable reference to an item from the `self.hashmap` object, and fails if it does not exist.
        /// Example:
        /// ```rs
        /// let q_void: &mut Ability = get_mut![Q::Void];
        /// ```
        macro_rules! get_mut {
            ($field1:ident::$field2:ident) => {{
                let key = AbilityLike::$field1(AbilityName::$field2);
                self.hashmap
                    .get_mut(&key)
                    .ok_or(format!("Failed to find field: {key:?}"))?
            }};
        }

        /// Merges the minimum and maximum damage of two items that should be contained in the `self.hashmap`,
        /// the result is added to `self.mergevec`.
        /// Example:
        /// ```rs
        /// merge![
        ///     (Q::Min, Q::Max),
        ///     (W::_2Min, W::_2),
        ///     (R::_4, R::_1Min),
        /// ]
        /// ```
        macro_rules! merge {
            ($($f1:ident::$v1:ident - $f2:ident::$v2:ident),+$(,)?) => {{
                $(
                    self.mergevec.push((
                        AbilityLike::$f1(AbilityName::$v1),
                        AbilityLike::$f2(AbilityName::$v2),
                    ));
                )+
            }};
        }

        /// Takes in two fields and returns a mutable reference to a new cloned value, that was
        /// already inserted to `self.hashmap`. The first enum is from where it is being cloned,
        /// and the second one is the new name it will have, and will be identical.
        /// Example:
        /// ```rs
        /// // Creates `Q::Max` and adds it to the hashmap
        /// let q_max_mut_ref: &mut Ability = clone_to![Q::_1Min => Q::Max];
        /// // If no changes were made, Q::Max have the same value as Q::_1Min because it was
        /// // cloned from there
        /// assert_eq!(q_max_mut_ref, get_mut![Q::_1Min]);
        /// ```
        macro_rules! clone_to {
            ($field3:ident::$field4:ident => $field1:ident::$field2:ident) => {{
                let clone_from = get!($field3::$field4).clone();
                insert!($field1::$field2, clone_from);
                get_mut!($field1::$field2)
            }};
        }

        /// Takes in an enum and a value that should be of type `Ability`, and inserts
        /// to `self.hashmap`.
        /// Example:
        /// ```rs
        /// let new_ability = Ability { ..default_ability };
        /// insert!(Q::_2, new_ability);
        /// ```
        macro_rules! insert {
            ($field1:ident::$field2:ident, $value:expr) => {{
                let temp = $value;
                self.hashmap.insert(AbilityLike::$field1(AbilityName::$field2), temp);
            }};
        }

        #old_block;

        // Verifies if any ability found has unknown damage and emits a warning
        // to the console so it can be fixed by the next time the generator runs
        self.hashmap
            .iter()
            .filter(|(_, value)| value.damage_type == DamageType::Unknown)
            .for_each(|(key, _)| {
                println!(
                    "[{name}]: Key {key:?} has unknown damage type",
                    name = self.data.name
                );
            });

        // Checks for minimum damage and maximum damage keys within the hashmap.
        // If it finds any key that is labeled as minimum damage, it will look
        // for keys that represent maximum damage. If it finds one, it will be
        // added to the mergevec, so it can be displayed in the tables as
        // `minimum damage - maximum damage`. If it doesn't find a maximum match,
        // a warning is emitted to the console and the key is skipped.
        let keys = self.hashmap.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            let index = key.ability_name() as u8;

            const MIN_I: u8 = AbilityName::Min as u8;
            const MIN_J: u8 = AbilityName::_8Min as u8;
            const MAX_I: u8 = AbilityName::Max as u8;
            const MAX_J: u8 = AbilityName::_8Max as u8;

            let min_range = MIN_I..=MIN_J;
            const MAX_MATCH: u8 = 1 + MAX_J - MAX_I;

            let make = key.from_fn();

            if min_range.contains(&index) {
                let mut found = false;
                let ability_name =
                    unsafe { std::mem::transmute::<_, AbilityName>(index + MAX_MATCH) };
                let ability_like = make(ability_name);
                if self.hashmap.contains_key(&ability_like) {
                    self.mergevec.push((key, ability_like));
                    found = true;
                }

                if !found {
                    println!(
                        "[{name}]: Found a min key: {key:?} with no max matches",
                        name = self.data.name
                    );
                }
            }
        }

        // Verifies if the mergevec makes sense. It means that the generated hashmap should
        // contain all keys that are present in the mergevec. If it doesn't, the function
        // returns a fail and prints a message to the console.
        if !self
            .mergevec
            .iter()
            .all(|(a, b)| self.hashmap.contains_key(a) && self.hashmap.contains_key(b))
        {
            println!(
                "{}: inconsistent data inserted in macro `merge!`.\nmerge_vec: {:?},\n`hashmap_keys: {:?}",
                self.data.name,
                self.mergevec,
                self.hashmap.keys().collect::<Vec<_>>()
            );
            return Err("Found inconsistent merge vec".into());
        }

        Ok(self.0.finish())
    }));

    TokenStream::from(quote!(#func))
}
