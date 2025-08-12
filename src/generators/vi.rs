use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_vi(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Max, Max), (1, 1, Minion1, Min));
    ability!(w, (1, 0, _1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
