use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_taliyah(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max),
        (3, 0, _4Min, Min)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (1, 0, _2Max, Max),
        (2, 0, _3Min, Min)
    );
}
