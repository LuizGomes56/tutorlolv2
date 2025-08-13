use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_reksai(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min), (0, 0, _2Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
