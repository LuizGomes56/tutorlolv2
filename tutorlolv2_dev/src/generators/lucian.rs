use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_lucian(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (0, 0, _1Min));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Min),
        (0, 3, _4Min),
        (0, 4, _5Max),
        (0, 5, _6Max)
    );
}
