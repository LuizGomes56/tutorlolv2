use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_milio(data: CdnChampion) -> Champion {
    ability!(q, (3, 0, _1Min, Min));
}
