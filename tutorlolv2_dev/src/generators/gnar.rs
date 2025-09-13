use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gnar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min), (0, 0, Mega, Min));
    ability!(w, (2, 0, _1, Min), (0, 0, _2Min, Min));
    ability!(e, (4, 0, _1Min, Min), (0, 0, _2Min, Min));
    ability!(r, (0, 0, _1Max, Max), (1, 1, _2Min, Min));
}
