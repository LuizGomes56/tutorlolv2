use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_xayah(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (1, 0, _3Min, Min),
        (1, 1, _4Max, Max)
    );
    ability!(e, (2, 0, _1Min, Min), (2, 1, _2Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
