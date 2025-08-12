use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_yasuo(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (2, 0, _2, Min),
        (2, 1, _3, Min),
        (2, 2, _4Max, Max)
    );
    ability!(r, (3, 0, _1Min, Min));
}
