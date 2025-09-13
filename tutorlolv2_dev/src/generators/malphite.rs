use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_malphite(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1, Min), (1, 0, _2Min, Min));
    ability!(e, (0, 1, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
