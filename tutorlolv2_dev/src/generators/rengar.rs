use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rengar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
}
