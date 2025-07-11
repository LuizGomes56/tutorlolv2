use super::*;

// #![stable] "06/19/2025" | "25.12"

#[generator_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
}
