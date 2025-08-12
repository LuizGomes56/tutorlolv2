use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_singed(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min), (2, 1, _2Max, Max));
    ability!(e, (0, 0, _1Min, Min));
}
