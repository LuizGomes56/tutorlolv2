use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[generator_macros::generator]
pub fn gen_jarvaniv(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
}
