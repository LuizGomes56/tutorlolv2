use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_irelia(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min, Min));
    ability!(w, (3, 0, _1Max, Max), (3, 1, Minion1, Min));
    ability!(e, (2, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
