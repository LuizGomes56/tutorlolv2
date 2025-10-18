use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_mordekaiser(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (0, 1, _2Min));
    ability!(e, (0, 0, _1Min));
}
