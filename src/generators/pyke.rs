use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min));
}
