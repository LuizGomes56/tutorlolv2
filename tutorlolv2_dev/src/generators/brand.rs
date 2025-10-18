use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_brand(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min));
    ability!(w, (0, 0, _1Max), (1, 0, _2Min));
    ability!(e, (1, 0, _1Min));
    ability!(r, (1, 0, _1Min), (1, 1, _2Max));
}
