use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_corki(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
    ability!(e, (0, 0, _1Min, Min), (0, 2, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min), (2, 0, _2Min, Min));
}
