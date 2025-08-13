use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_hecarim(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(w, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(e, (3, 0, _1Max, Max), (3, 1, Minion1, Min));
    ability!(r, (0, 0, _1Min, Min));
}
