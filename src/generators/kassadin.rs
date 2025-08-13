use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_kassadin(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (1, 0, _2, Min),
        (1, 1, _3, Min),
        (1, 2, _4Max, Max)
    );
}
