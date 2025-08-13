use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_bard(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void, Min));
}
