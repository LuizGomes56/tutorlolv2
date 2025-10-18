use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gwen(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Min),
        (0, 3, _4Min),
        (1, 0, _5Max),
        (1, 1, _6Max),
        (1, 2, Minion1),
        (1, 3, Minion2)
    );
    ability!(e, (0, 1, _1));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (3, 0, _3Max),
        (3, 1, _4Max),
        (3, 2, _5Max)
    );
}
