use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kayn(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Min, Min),
        (1, 1, _4Max, Max),
        (2, 0, Monster1, Min),
        (2, 1, Monster2, Min)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(r, (3, 0, _1Min, Min));
}
