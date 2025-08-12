use super::*;

// #![stable] "08/07/2025" | "25.15"
/// * Column Cell Template Exibition = `{LEFT} - {RIGHT}`
/// * Q `{LEFT}` represents initial damage, and `{RIGHT}` the total.
/// * Q1 represents the damage of each secondary explosion

#[generator_macros::generator]
pub fn gen_neeko(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q", Min),
        (1, 0, "Q_MONSTER", Min),
        (2, 0, "Q1", Min),
        (2, 1, "Q_MAX", Max)
    );
    ability!(w, (1, 0, "W", Min));
    ability!(e, (0, 0, "E", Min));
    ability!(r, (2, 0, "R", Min));
    merge_ability!("Q");
}
