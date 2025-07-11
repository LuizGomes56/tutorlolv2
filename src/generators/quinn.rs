use super::*;

// #![stable] "06/19/2025" | "25.12"

#[generator_macros::generator]
pub fn gen_quinn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
}
