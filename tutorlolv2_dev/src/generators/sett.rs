use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sett(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (0, 1, _2));
    ability!(w, (1, 0, _1Min));
    ability!(e, (0, 0, Monster1), (0, 1, _1Min));
    ability!(r, (1, 0, _1Min), (1, 1, _2Min));
}
