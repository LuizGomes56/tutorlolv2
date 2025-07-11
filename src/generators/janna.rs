use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_janna(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q_0_0_0_MINIMUM", Target::MINIMUM),
        (0, 1, "Q_0_0_1_BONUS", Target::MINIMUM),
        (0, 2, "Q_0_0_2_MAXIMUM", Target::MAXIMUM)
    );
    ability!(w, (1, 0, "W_0_1_0", Target::MINIMUM));
    ability!(e, (2, 0, "E_0_2_0_BONUS", Target::MINIMUM));
}
