use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ivern(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (2, 0, _1), (3, 0, _2));
    ability!(e, (1, 0, _1Min));
}
