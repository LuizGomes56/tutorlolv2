use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_leona(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1));
    ability!(w, (0, 2, _1Min), (1, 0, _2Min));
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
