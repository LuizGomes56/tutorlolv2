use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_gwen(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Min, Min),
        (0, 3, _4Min, Min),
        (1, 0, _5Max, Max),
        (1, 1, _6Max, Max),
        (1, 2, Minion1, Min),
        (1, 3, Minion2, Min)
    );
    ability!(e, (0, 1, _1, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (3, 0, _3Max, Max),
        (3, 1, _4Max, Max),
        (3, 2, _5Max, Max)
    );
}
