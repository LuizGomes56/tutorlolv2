use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_shaco(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1, Min));
    ability!(
        w,
        (1, 0, Monster1, Min),
        (1, 1, _1Max, Max),
        (1, 2, Monster2, Min),
        (1, 3, _2Min, Min),
        (1, 4, Monster3, Min)
    );
    ability!(e, (0, 0, _1Max, Max), (0, 1, _2Min, Min));
    ability!(
        r,
        (3, 0, _1Max, Max),
        (3, 1, _2Min, Min),
        (4, 0, _3Min, Min)
    );
}
