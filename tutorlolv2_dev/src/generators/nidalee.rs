use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nidalee(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max, Max),
        (0, 1, Minion1, Min),
        (0, 0, _2Max, Max),
        (0, 1, _3Max, Max),
        (0, 2, _4Max, Max),
        (0, 3, Minion2, Min),
        (1, 0, _5Max, Max),
        (1, 1, Minion3, Min)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 0, _3Min, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
}
