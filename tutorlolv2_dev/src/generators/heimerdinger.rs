use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_heimerdinger(data: CdnChampion) -> Champion {
    ability!(
        w,
        (0, 0, _1, Min),
        (1, 0, _1Max, Max),
        (1, 1, _2Max, Max),
        (1, 2, _2, Min),
        (1, 3, _3, Min),
        (1, 4, _3Max, Max),
        (1, 5, _4Max, Max),
        (0, 0, _5Max, Max),
        (0, 1, _6Max, Max),
        (0, 2, _4, Min),
        (0, 3, _5, Min),
        (0, 4, _6, Min),
        (2, 0, _7Min, Min)
    );
    ability!(e, (0, 0, _1Min, Min), (2, 0, _2Min, Min));
}
