use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gangplank(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (1, 0, _1, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (0, 2, _3Max, Max),
        (1, 0, _4Max, Max),
        (1, 1, _5Min, Min),
        (2, 0, _6Max, Max),
        (2, 1, _7Max, Max)
    );
}
