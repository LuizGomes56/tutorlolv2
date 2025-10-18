use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_hecarim(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (0, 1, _2Min));
    ability!(w, (0, 0, _1Min), (0, 1, _2Max));
    ability!(e, (3, 0, _1Max), (3, 1, Minion1));
    ability!(r, (0, 0, _1Min));
}
