use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rammus(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min));
    ability!(e, (0, 0, Monster1));
    ability!(r, (0, 0, _1Min));
}
