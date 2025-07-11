use super::*;

// #![stable] "06/11/2025" | "25.11"

#[generator_macros::generator]
pub fn gen_ashe(data: CdnChampion) -> Champion {
    ability!(
        q,
        (2, 0, "Q", Target::MINIMUM),
        (2, 1, "Q_MAX", Target::MAXIMUM)
    );
    ability!(w, (0, 1, "W", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("Q");
}
