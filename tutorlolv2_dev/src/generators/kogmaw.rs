use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kogmaw(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 1, _1));
    ability!(e, (1, 0, _1Min));
    ability!(r, (0, 0, _1Max), (0, 1, Minion1));
}
