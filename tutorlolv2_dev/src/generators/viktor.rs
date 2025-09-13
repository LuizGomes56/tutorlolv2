use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_viktor(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (2, 0, _2Min, Min),
        (2, 1, _3Max, Max)
    );
    ability!(
        e,
        (0, 0, _1Min, Min),
        (1, 0, _2Min, Min),
        (1, 1, _3Max, Max)
    );
    ability!(
        r,
        (0, 0, _1Min, Min),
        (5, 0, _2Min, Min),
        (5, 1, _3Max, Max)
    );
}
