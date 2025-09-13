use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lucian(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Min, Min),
        (0, 3, _4Min, Min),
        (0, 4, _5Max, Max),
        (0, 5, _6Max, Max)
    );
}
