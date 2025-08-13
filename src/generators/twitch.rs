use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_twitch(data: CdnChampion) -> Champion {
    ability!(
        e,
        (1, 0, _1Min, Min),
        (2, 0, _2Max, Max),
        (2, 1, Minion1, Min),
        (2, 2, _3Min, Min)
    );
    ability!(r, (0, 0, _1, Min));
}
