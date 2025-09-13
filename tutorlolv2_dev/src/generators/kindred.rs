use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kindred(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (3, 0, _1Min, Min), (3, 1, Monster1, Min));
    ability!(e, (2, 0, _1Min, Min), (2, 1, _2Min, Min));
}
