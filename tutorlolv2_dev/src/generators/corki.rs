use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_corki(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (1, 0, _1Min), (1, 1, _2Max));
    ability!(e, (0, 0, _1Min), (0, 2, _2Max));
    ability!(r, (0, 0, _1Min), (2, 0, _2Min));
}
