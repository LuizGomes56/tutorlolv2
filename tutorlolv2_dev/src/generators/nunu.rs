use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nunu(data: CdnChampion) -> Champion {
    ability!(q, (1, 1, _1Min), (2, 2, _2Min));
    ability!(
        w,
        (2, 0, _1Max),
        (2, 1, Minion1),
        (4, 0, _2Max),
        (4, 1, Minion2)
    );
    ability!(
        e,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (1, 0, _3Min),
        (3, 0, _4Max)
    );
    ability!(r, (2, 0, _1Min));
}
