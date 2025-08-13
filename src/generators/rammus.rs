use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rammus(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(e, (0, 0, Monster1, Min));
    ability!(r, (0, 0, _1Min, Min));
}
