use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_vi(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Max), (1, 1, Minion1));
    ability!(w, (1, 0, _1));
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
