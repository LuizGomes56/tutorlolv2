use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_singed(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min), (2, 1, _2Max));
    ability!(e, (0, 0, _1Min));
}
