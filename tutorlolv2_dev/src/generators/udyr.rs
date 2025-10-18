use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_udyr(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (0, 1, _2Max), (1, 1, _3));
    ability!(r, (1, 0, _1Min), (1, 2, _2Max));
}
