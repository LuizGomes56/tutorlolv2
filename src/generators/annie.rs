use super::*;

// #![stable] "06/11/2025" | "25.11"
// #![unsupported] Tibbers
// #![todo] Add Tibbers

#[generator_macros::generator]
pub fn gen_annie(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
    ability!(r, (1, 0, "R", Target::MINIMUM));
}
