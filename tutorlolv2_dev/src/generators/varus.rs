use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_varus(data: CdnChampion) -> Champion {
    ability!(
        q,
        (2, 0, _1Max, Max),
        (2, 1, _2Max, Max),
        (2, 2, Minion1, Min),
        (2, 3, Minion2, Min)
    );
    ability!(
        w,
        (0, 0, _1Max, Max),
        (0, 1, Minion1, Min),
        (1, 0, _2, Min),
        (1, 1, _3, Min),
        (1, 2, _4, Min),
        (1, 3, _5, Min),
        (4, 0, _6, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
