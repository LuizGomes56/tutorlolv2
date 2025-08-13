use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ksante(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (1, 0, _1, Min),
        (1, 1, _2, Min),
        (1, 2, _3Max, Max),
        (4, 0, Monster1, Min),
        (4, 1, _4Min, Min)
    );
    ability!(
        r,
        (0, 0, _1Min, Min),
        (3, 0, _2Min, Min),
        (3, 1, _3Max, Max)
    );
}
