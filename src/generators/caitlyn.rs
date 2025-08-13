use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_caitlyn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(w, (2, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (2, 0, _1Min, Min));
}
