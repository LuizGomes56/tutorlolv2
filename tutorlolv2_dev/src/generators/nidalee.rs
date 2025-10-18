use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nidalee(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max),
        (0, 1, Minion1),
        (0, 0, _2Max),
        (0, 1, _3Max),
        (0, 2, _4Max),
        (0, 3, Minion2),
        (1, 0, _5Max),
        (1, 1, Minion3)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (0, 0, _3Min)
    );
    ability!(e, (0, 0, _1Min));
}
