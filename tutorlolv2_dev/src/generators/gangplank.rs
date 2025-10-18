use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gangplank(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(e, (1, 0, _1));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max),
        (1, 0, _4Max),
        (1, 1, _5Min),
        (2, 0, _6Max),
        (2, 1, _7Max)
    );
}
