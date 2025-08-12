use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kaisa(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Min, Min),
        (2, 0, _2Max, Max),
        (3, 0, _3Min, Min),
        (3, 1, _4Max, Max)
    );
    ability!(w, (0, 0, _1Min, Min));
}
