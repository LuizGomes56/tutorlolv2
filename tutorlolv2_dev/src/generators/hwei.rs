use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_hwei(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 0, _2Min),
        (1, 0, _3Max),
        (1, 1, _4Max),
        (0, 0, _5Min),
        (1, 0, _6Min),
        (1, 1, _7Max),
        (1, 2, _8Max)
    );
    ability!(w, (0, 0, _1), (0, 2, _2Max), (1, 0, _3));
    ability!(
        e,
        (0, 1, _1Min),
        (0, 0, _2Min),
        (0, 0, _3Min)
    );
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (1, 0, _3Min),
        (1, 1, _4Max)
    );
}
