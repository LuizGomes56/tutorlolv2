use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// ## Provides useful macros to extract data from CDN API
///
/// Macros: `ability!`, `passive!`
///
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
///
/// ### `passive!`: fn `extract_passive_damage` in a more convenient way
///
/// If more than 3 arguments are supplied, a tuple with (Scalling, Postfix) is expected
/// Otherwise, just the name of the key, its coordinates and the target vector are expected
///
/// ```
/// passive!("P", (0, 0), Target::MINIMUM);
/// passive!("P", (0, 0), Target::MINIMUM, (None, Some("ENEMY_MAX_HEALTH")));
/// passive!("P", (0, 0), Target::MAXIMUM, (Some(<usize>scalling), Some("POSTFIX")));
/// ```
#[proc_macro_attribute]
pub fn writer(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as syn::ItemFn);

    let map_decl = quote! {
        let mut abilities = HashMap::<String, Ability>::new();

        #[allow(unused_macros)]
        macro_rules! ability {
            ($field:ident, $idx:literal, $(($a:literal, $b:literal, $c:literal, $d:expr)),* $(,)?) => {{
                let pattern = [$(($a, $b, $c, $d)),*];
                extract_ability_damage(&data.abilities.$field[$idx], &mut abilities, &pattern);
            }};
            ($field:ident, $(($a:literal, $b:literal, $c:literal, $d:expr)),* $(,)?) => {{
                let pattern = [$(($a, $b, $c, $d)),*];
                extract_ability_damage(&data.abilities.$field[0], &mut abilities, &pattern);
            }};
        }

        #[allow(unused_macros)]
        macro_rules! passive {
            ($key:literal, ($i:literal, $j:literal), $target:expr) => {{
                extract_passive_damage(&data, ($i, $j), None, None, &$target, $key, &mut abilities);
            }};
            ($key:literal, ($i:literal, $j:literal), $target:expr, ($scaling:expr, $postfix:expr)) => {{
                extract_passive_damage(
                    &data,
                    ($i, $j),
                    $postfix,
                    $scaling,
                    &$target,
                    $key,
                    &mut abilities,
                );
            }};
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
