use super::*;

// #![stable] "08/07/2025" | "25.15"
/// * Column Cell Template Exibition = `{LEFT} - {RIGHT}`
/// * Q `{LEFT}` represents initial damage, and `{RIGHT}` the total.
/// * Q1 represents the damage of each secondary explosion

#[tutorlolv2_macros::generator]
pub fn gen_neeko(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Void),
        (1, 0, Monster),
        (2, 0, _1),
        (2, 1, _1Max)
    );
    ability!(w, (1, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(r, (2, 0, Void));
    // merge_ability!(Q::_1, Q::_1Max);
}
