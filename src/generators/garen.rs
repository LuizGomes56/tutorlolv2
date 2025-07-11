use super::*;

// #![stable] "06/18/2025" | "25.11"

#[generator_macros::generator]
pub fn gen_garen(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E", Target::MINIMUM),
        (2, 0, "E_MAX", Target::MAXIMUM)
    );
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("E");
}
