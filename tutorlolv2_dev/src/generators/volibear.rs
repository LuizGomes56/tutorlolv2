use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_volibear(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1));
    ability!(w, (0, 0, _1Min));
    ability!(e, (1, 0, _1Min));
    ability!(r, (4, 0, _1Min));
}
