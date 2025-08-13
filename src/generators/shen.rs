use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_shen(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, Monster1, Min),
        (2, 0, _1, Min),
        (2, 1, _2Max, Max),
        (3, 0, _3, Min),
        (3, 1, _4Max, Max)
    );
    ability!(e, (0, 0, _1Min, Min));
}
