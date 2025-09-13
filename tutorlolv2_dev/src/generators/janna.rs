use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_janna(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min), (0, 1, _2Max, Max), (0, 2, Minion1, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (2, 0, _1, Min));
}
