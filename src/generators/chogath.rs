use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_chogath(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 2, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
}
