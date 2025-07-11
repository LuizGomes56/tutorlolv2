use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_draven(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0_BONUS", Target::MINIMUM));
    ability!(e, (0, 0, "E_0_0_0", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1_MAXIMUM", Target::MAXIMUM),
        (3, 0, "R_0_3_0_MINIMUM", Target::MINIMUM),
        (3, 1, "R_0_3_1_MAXIMUM", Target::MAXIMUM)
    );
}
