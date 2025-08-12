use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kalista(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (1, 0, _1, Min), (1, 1, _2Max, Max));
    ability!(e, (1, 0, _1Min, Min), (1, 1, _2Min, Min));
}
