use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lulu(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (1, 0, _3Min, Min),
        (1, 1, _4Min, Min),
        (1, 2, _5Max, Max),
        (1, 3, _6Max, Max)
    );
    ability!(e, (1, 0, _1Min, Min));
}
