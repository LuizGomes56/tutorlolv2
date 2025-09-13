use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_zilean(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
}
