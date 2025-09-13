use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sona(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (1, 0, _2, Min));
    ability!(w, (2, 0, Minion1, Min));
    ability!(r, (0, 0, _1Min, Min));
}
