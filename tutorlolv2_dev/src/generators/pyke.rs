use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_pyke(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min));
}
