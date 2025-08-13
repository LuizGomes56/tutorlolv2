use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_belveth(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (2, 0, _2Min, Min),
        (2, 1, Monster1, Min),
        (2, 2, Monster2, Min)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (3, 0, Monster1, Min),
        (3, 1, Monster2, Min),
        (5, 0, _2Max, Max),
        (5, 1, Minion1, Min)
    );
    ability!(
        r,
        (1, 0, _1Min, Min),
        (2, 0, _2, Min),
        (2, 1, Monster1, Min)
    );
}
