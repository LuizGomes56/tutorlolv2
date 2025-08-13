use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zyra(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
