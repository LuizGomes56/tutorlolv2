use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_viego(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1, Min),
        (0, 1, _1Max, Max),
        (0, 2, Monster2, Min),
        (0, 3, Minion1, Min),
        (3, 0, _2, Min),
        (3, 1, _3, Min)
    );
    ability!(w, (1, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
