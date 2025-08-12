use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_seraphine(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 0, _2Max, Max));
    ability!(e, (0, 1, _1Min, Min), (0, 2, _2Min, Min));
    ability!(r, (0, 1, _1Min, Min));
}
