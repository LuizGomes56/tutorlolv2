use super::*;

// #![auto_generated]

#[generator_macros::generator]
pub fn gen_illaoi(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q_0_0_0", Target::MINIMUM));
    ability!(
        w,
        (1, 0, "W_0_1_0_BONUS", Target::MINIMUM),
        (1, 1, "W_0_1_1_MINIMUM", Target::MINIMUM)
    );
    ability!(e, (1, 0, "E_0_1_0", Target::MINIMUM));
    ability!(r, (0, 0, "R_0_0_0", Target::MINIMUM));
}
