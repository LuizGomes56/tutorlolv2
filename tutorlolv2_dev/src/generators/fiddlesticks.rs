use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_fiddlesticks(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 1, _1Min),
        (0, 2, Minion1),
        (2, 0, _2Max),
        (2, 1, _3Max)
    );
    ability!(
        w,
        (4, 0, _1Min),
        (4, 1, _2Min),
        (4, 2, _3Min),
        (4, 3, _4Max)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min), (0, 1, _2Max));
}
