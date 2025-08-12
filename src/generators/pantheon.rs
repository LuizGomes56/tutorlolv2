use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_pantheon(data: CdnChampion) -> Champion {
    ability!(
        q,
        (4, 0, _1Min, Min),
        (4, 1, _2Min, Min),
        (4, 2, _3Max, Max),
        (4, 3, _4Max, Max),
        (5, 0, _5Max, Max),
        (5, 1, _6Min, Min)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (3, 0, _1Min, Min));
    ability!(r, (3, 0, _1Min, Min), (3, 1, _2Min, Min));
}
