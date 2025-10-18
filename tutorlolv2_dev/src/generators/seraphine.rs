use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_seraphine(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (1, 0, _2Max));
    ability!(e, (0, 1, _1Min), (0, 2, _2Min));
    ability!(r, (0, 1, _1Min));
}
