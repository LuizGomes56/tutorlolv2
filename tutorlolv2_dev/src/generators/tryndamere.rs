use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_tryndamere(data: CdnChampion) -> Champion {
    ability!(q, (1, 1, _1, Min));
    ability!(w, (1, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
}
