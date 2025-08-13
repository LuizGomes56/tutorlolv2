use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_sion(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max, Max),
        (0, 1, _2Max, Max),
        (0, 2, Minion1, Min),
        (3, 0, _3Max, Max),
        (3, 1, Monster1, Min),
        (3, 2, Minion2, Min),
        (3, 3, Monster2, Min)
    );
    ability!(w, (3, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (2, 0, _1Max, Max), (2, 1, Minion1, Min));
}
