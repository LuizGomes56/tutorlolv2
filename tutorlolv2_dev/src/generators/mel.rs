use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_mel(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 3, _3Max),
        (0, 4, _4Max)
    );
    ability!(w, (1, 0, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (1, 0, _2Min),
        (1, 1, _3Min),
        (1, 2, _4Min),
        (2, 0, _5Min),
        (2, 1, _6Min)
    );
    ability!(
        r,
        (0, 0, _1Min),
        (2, 0, _2Max),
        (2, 1, _3Min)
    );
}
