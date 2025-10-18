use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_jhin(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (1, 0, _2), (1, 1, _3Max));
    ability!(w, (0, 0, _1Min), (0, 1, _2Min));
    ability!(e, (1, 0, _1Min), (1, 1, _2Min));
    ability!(
        r,
        (1, 0, _1Max),
        (1, 1, Minion1),
        (2, 0, _2Max),
        (2, 1, Minion2)
    );
}
