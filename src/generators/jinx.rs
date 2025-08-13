use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jinx(data: CdnChampion) -> Champion {
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (1, 0, _1Max, Max),
        (1, 1, Minion1, Min),
        (2, 0, _2Max, Max),
        (2, 1, Minion2, Min)
    );
}
