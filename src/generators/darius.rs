use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_darius(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(w, (0, 0, _1, Min));
    ability!(r, (0, 0, _1, Min), (0, 1, _2Max, Max), (0, 2, _3Min, Min));
}
