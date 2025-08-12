use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_renata(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min));
}
