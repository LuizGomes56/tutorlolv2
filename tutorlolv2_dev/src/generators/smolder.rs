use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_smolder(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max),
        (0, 1, _2Max),
        (0, 2, _3Min),
        (3, 0, _4Max),
        (3, 1, _5Max),
        (3, 2, _6Min)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max)
    );
    ability!(e, (0, 0, _1Max), (0, 1, _2Min));
    ability!(r, (0, 0, _1Max), (0, 1, _2Min));
}
