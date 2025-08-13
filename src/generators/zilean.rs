use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zilean(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
}
