use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_missfortune(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(r, (0, 0, _1Max, Max));
}
