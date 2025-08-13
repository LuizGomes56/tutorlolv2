use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_leesin(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 0, _2Max, Max),
        (0, 1, Minion1, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
}
