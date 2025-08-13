use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_evelynn(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Min, Min),
        (2, 0, _2, Min),
        (2, 1, _3, Min),
        (5, 0, _4Min, Min),
        (5, 1, _5Max, Max),
        (5, 2, _6Max, Max)
    );
    ability!(w, (2, 0, _1, Min));
    ability!(e, (0, 0, _1Min, Min), (0, 0, _2Min, Min));
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
}
