use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yone(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max)
    );
    ability!(e, (3, 0, _1Min));
    ability!(
        r,
        (1, 0, _1Min),
        (1, 1, _2Min),
        (1, 2, _3Max)
    );
}
