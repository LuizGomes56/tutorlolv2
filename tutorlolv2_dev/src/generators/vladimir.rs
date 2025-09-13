use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_vladimir(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min, Min), (2, 0, _2Max, Max));
    ability!(w, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
    ability!(e, (4, 0, _1Max, Max), (4, 1, Minion1, Min));
    ability!(r, (1, 1, _1Min, Min));
}
