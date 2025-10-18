use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nocturne(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (1, 0, _2));
    ability!(e, (0, 0, _1Min), (0, 1, _2Max));
    ability!(r, (2, 0, _1Min));
}
