use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_hwei(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 0, _2Min, Min),
        (1, 0, _3Max, Max),
        (1, 1, _4Max, Max),
        (0, 0, _5Min, Min),
        (1, 0, _6Min, Min),
        (1, 1, _7Max, Max),
        (1, 2, _8Max, Max)
    );
    ability!(w, (0, 0, _1, Min), (0, 2, _2Max, Max), (1, 0, _3, Min));
    ability!(
        e,
        (0, 1, _1Min, Min),
        (0, 0, _2Min, Min),
        (0, 0, _3Min, Min)
    );
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Min, Min),
        (1, 1, _4Max, Max)
    );
}
