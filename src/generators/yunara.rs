use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yunara(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1, Min),
        (0, 1, _2Max, Max),
        (0, 3, _3, Min),
        (0, 4, _4Max, Max),
        (2, 0, _5, Min)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max)
    );
    ability!(r, (1, 0, _1Min, Min));
}
