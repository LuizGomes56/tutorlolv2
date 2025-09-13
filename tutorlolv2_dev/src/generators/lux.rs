use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lux(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (2, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
