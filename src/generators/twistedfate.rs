use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_twistedfate(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (1, 0, _1Min, Min),
        (2, 0, _2Min, Min),
        (5, 0, _3Min, Min)
    );
    ability!(e, (0, 1, _1, Min));
}
