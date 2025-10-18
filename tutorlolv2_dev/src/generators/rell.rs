use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rell(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 0, _1Min), (0, 0, _2));
    ability!(e, (1, 0, _1));
    ability!(r, (0, 0, _1Min), (0, 1, _2Max));
}
