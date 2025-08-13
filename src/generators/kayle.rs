use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kayle(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1, Min), (2, 0, _2Min, Min));
    ability!(r, (1, 0, _1Min, Min));
}
