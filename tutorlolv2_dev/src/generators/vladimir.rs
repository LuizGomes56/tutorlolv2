use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_vladimir(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min), (2, 0, _2Max));
    ability!(w, (1, 0, _1Min), (1, 1, _2Max));
    ability!(e, (4, 0, _1Max), (4, 1, Minion1));
    ability!(r, (1, 1, _1Min));
}
