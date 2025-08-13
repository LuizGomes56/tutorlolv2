use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_khazix(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Max, Max), (0, 1, _2Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
}
