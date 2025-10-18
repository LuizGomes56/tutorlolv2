use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rumble(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max),
        (0, 3, _4Max),
        (1, 0, _5Min),
        (1, 1, _6Min),
        (1, 2, _7Max),
        (1, 3, _8Max)
    );
    ability!(
        e,
        (0, 0, _1Min),
        (0, 4, _2Max),
        (1, 0, _3Min),
        (1, 3, _4Max)
    );
    ability!(
        r,
        (1, 0, _1Min),
        (1, 1, _2Max),
        (1, 2, Minion1)
    );
}
