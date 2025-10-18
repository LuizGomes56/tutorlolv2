use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_jarvaniv(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min));
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
