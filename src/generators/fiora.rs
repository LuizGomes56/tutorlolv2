use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[generator_macros::generator]
pub fn gen_fiora(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(w, (1, 0, "W", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
}
