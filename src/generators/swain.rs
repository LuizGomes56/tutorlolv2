use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_swain(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min), (0, 1, _2Min, Min), (0, 2, _3Max, Max));
    ability!(w, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(e, (1, 0, _1Min, Min));
    ability!(r, (2, 1, _1Min, Min), (0, 0, _2Min, Min));
}
