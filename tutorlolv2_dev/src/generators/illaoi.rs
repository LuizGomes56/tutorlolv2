use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_illaoi(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min));
    ability!(w, (3, 0, _1), (3, 1, Minion1));
    ability!(e, (3, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
