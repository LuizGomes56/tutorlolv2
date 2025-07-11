use super::*;

// #![stable] "06/11/2025" | "25.11"
// #![unsupported] [P] BARD_STACKS (Meeps)

#[generator_macros::generator]
pub fn gen_bard(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
}
