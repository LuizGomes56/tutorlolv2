use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lulu(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (1, 0, _3Min),
        (1, 1, _4Min),
        (1, 2, _5Max),
        (1, 3, _6Max)
    );
    ability!(e, (1, 0, _1Min));
}
