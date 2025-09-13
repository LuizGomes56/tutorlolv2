use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_taric(data: CdnChampion) -> Champion {
    ability!(e, (0, 0, _1Min, Min));
}
