use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_leona(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(w, (0, 2, _1Min, Min), (1, 0, _2Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
