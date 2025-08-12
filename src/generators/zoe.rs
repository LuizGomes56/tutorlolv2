use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zoe(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Max, Max), (0, 1, Minion1, Min));
    ability!(w, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
    ability!(e, (1, 0, _1Min, Min), (2, 0, _2, Min), (2, 1, _3Max, Max));
}
