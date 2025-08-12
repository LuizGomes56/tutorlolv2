use super::*;

// #![stable] "08/07/2025" | "25.15"
/// * Column Cell Template Exibition = `{LEFT} - {RIGHT}`
/// * Q `{LEFT}` represents initial damage, and `{RIGHT}` the total.
/// * Q1 represents the damage of each secondary explosion

#[generator_macros::generator]
pub fn gen_neeko(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Void, Min),
        (1, 0, Monster, Min),
        (2, 0, _1, Min),
        (2, 1, _1Max, Max)
    );
    ability!(w, (1, 0, Void, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(r, (2, 0, Void, Min));
    merge_ability!((Q, _1), (Q, _1Max));
}
