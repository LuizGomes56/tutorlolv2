use super::*;

// #![stable] "06/18/2025" | "25.11"

#[generator_macros::generator]
pub fn gen_gnar(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 1, "Q", Target::MINIMUM),
        (0, 0, "Q_MAX", Target::MAXIMUM),
    );
    ability!(q, 1, (0, 0, "Q_MEGA", Target::MINIMUM));
    ability!(w, (1, 0, "W", Target::MINIMUM),);
    ability!(w, 1, (0, 0, "W_MEGA", Target::MINIMUM));
    ability!(e, (2, 0, "E", Target::MINIMUM),);
    ability!(e, 1, (0, 0, "E_MEGA", Target::MINIMUM));
    ability!(
        r,
        (1, 0, "R", Target::MINIMUM),
        (2, 0, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("Q");
    merge_ability!("R");
}
