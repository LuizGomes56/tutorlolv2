use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nunu(data: CdnChampion) -> Champion {
    ability!(q, (1, 1, _1Min, Min), (2, 2, _2Min, Min));
    ability!(
        w,
        (2, 0, _1Max, Max),
        (2, 1, Minion1, Min),
        (4, 0, _2Max, Max),
        (4, 1, Minion2, Min)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Min, Min),
        (3, 0, _4Max, Max)
    );
    ability!(r, (2, 0, _1Min, Min));
}
