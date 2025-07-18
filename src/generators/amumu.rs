use super::*;

// ! #![unstable] [W] "06/11/2025" | "25.11"
// #![preserve]
// #![todo] W should join in between scallings a "*", not a "+"

#[generator_macros::generator]
pub fn gen_amumu(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
}
