use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_smolder(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max, Max),
        (0, 1, _2Max, Max),
        (0, 2, _3Min, Min),
        (3, 0, _4Max, Max),
        (3, 1, _5Max, Max),
        (3, 2, _6Min, Min)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max)
    );
    ability!(e, (0, 0, _1Max, Max), (0, 1, _2Min, Min));
    ability!(r, (0, 0, _1Max, Max), (0, 1, _2Min, Min));
}
