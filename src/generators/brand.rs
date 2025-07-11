use super::*;

// ! #![unstable] [E] "06/11/2025" | "25.11"
// #![preserve]
// #![todo] Create generator/macro for his passive

#[generator_macros::generator]
pub fn gen_brand(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        w,
        (0, 0, "W", Target::MINIMUM),
        (1, 0, "W_MAX", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R", Target::MINIMUM),
        (0, 1, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("W");
    merge_ability!("R");
}
