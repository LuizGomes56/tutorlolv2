use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_orianna(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
