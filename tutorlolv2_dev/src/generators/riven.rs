use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_riven(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min), (2, 1, _2Max));
    ability!(w, (0, 0, _1Min));
    ability!(r, (0, 0, _1Max), (0, 1, Minion1));
}
