use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_fiddlesticks(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 1, _1Min, Min),
        (0, 2, Minion1, Min),
        (2, 0, _2Max, Max),
        (2, 1, _3Max, Max)
    );
    ability!(
        w,
        (4, 0, _1Min, Min),
        (4, 1, _2Min, Min),
        (4, 2, _3Min, Min),
        (4, 3, _4Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
}
