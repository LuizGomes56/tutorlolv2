use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_thresh(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 0, _2, Min), (1, 1, _3, Min));
    ability!(r, (0, 0, _1Min, Min));
}
