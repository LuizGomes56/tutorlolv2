use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_taric(data: CdnChampion) -> Champion {
    ability!(e, (0, 0, _1Min, Min));
}
