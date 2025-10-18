use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_twistedfate(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (1, 0, _1Min),
        (2, 0, _2Min),
        (5, 0, _3Min)
    );
    ability!(e, (0, 1, _1));
}
