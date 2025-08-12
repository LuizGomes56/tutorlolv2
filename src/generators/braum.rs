use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_braum(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 1, _1Min, Min));
    ability!(r, (1, 0, _1Min, Min));
}
