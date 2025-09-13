use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yone(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max)
    );
    ability!(e, (3, 0, _1Min, Min));
    ability!(
        r,
        (1, 0, _1Min, Min),
        (1, 1, _2Min, Min),
        (1, 2, _3Max, Max)
    );
}
