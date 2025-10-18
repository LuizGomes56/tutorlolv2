use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_urgot(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (1, 0, _1Min));
    ability!(e, (1, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
