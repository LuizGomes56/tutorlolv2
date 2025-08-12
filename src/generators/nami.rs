use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nami(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (1, 1, _1Min, Min), (1, 2, Minion1, Min));
    ability!(e, (0, 0, _1, Min), (0, 2, _2, Min));
    ability!(r, (0, 0, _1Min, Min));
}
