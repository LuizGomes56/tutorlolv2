use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_riven(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min), (2, 1, _2Max, Max));
    ability!(w, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Max, Max), (0, 1, Minion1, Min));
}
