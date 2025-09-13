use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rakan(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(r, (0, 1, _1Min, Min));
}
