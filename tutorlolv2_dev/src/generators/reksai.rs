use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_reksai(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (0, 0, _2Min));
    ability!(w, (0, 0, _1Min));
    ability!(e, (0, 0, _1Min), (1, 0, _2Min));
    ability!(r, (0, 0, _1Min));
}
