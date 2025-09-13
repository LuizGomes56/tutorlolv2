use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_mel(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 3, _3Max, Max),
        (0, 4, _4Max, Max)
    );
    ability!(w, (1, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Min, Min),
        (1, 2, _4Min, Min),
        (2, 0, _5Min, Min),
        (2, 1, _6Min, Min)
    );
    ability!(
        r,
        (0, 0, _1Min, Min),
        (2, 0, _2Max, Max),
        (2, 1, _3Min, Min)
    );
}
