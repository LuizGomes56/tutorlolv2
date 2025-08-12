use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_blitzcrank(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
}
