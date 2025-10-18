use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_chogath(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 0, _1Min));
    ability!(e, (0, 0, _1Min), (0, 2, _2Max));
    ability!(r, (0, 0, _1Min), (0, 1, _2Min));
}
