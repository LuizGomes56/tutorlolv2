use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_zeri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (1, 0, _1, Min), (1, 1, _2Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
