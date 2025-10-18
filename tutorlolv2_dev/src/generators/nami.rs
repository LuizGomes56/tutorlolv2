use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nami(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (1, 1, _1Min), (1, 2, Minion1));
    ability!(e, (0, 0, _1), (0, 2, _2));
    ability!(r, (0, 0, _1Min));
}
