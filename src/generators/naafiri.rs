use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_naafiri(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max),
        (0, 3, _4Max, Max),
        (1, 1, _5, Min),
        (1, 2, _6Max, Max),
        (1, 3, _7, Min),
        (1, 4, _8Max, Max)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max)
    );
    ability!(r, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
}
