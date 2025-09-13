use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_rumble(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max),
        (0, 3, _4Max, Max),
        (1, 0, _5Min, Min),
        (1, 1, _6Min, Min),
        (1, 2, _7Max, Max),
        (1, 3, _8Max, Max)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 4, _2Max, Max),
        (1, 0, _3Min, Min),
        (1, 3, _4Max, Max)
    );
    ability!(
        r,
        (1, 0, _1Min, Min),
        (1, 1, _2Max, Max),
        (1, 2, Minion1, Min)
    );
}
