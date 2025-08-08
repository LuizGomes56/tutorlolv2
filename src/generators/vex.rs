use super::*;

// #![stable] "08/07/2025" | "25.15"

#[generator_macros::generator]
pub fn gen_vex(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R_MINION", Target::MINIMUM),
        (2, 0, "R", Target::MINIMUM),
        (2, 1, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("R");
}
