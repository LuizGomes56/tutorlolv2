use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_heimerdinger(data: CdnChampion) -> Champion {
    ability!(
        w,
        (0, 0, _1),
        (1, 0, _1Max),
        (1, 1, _2Max),
        (1, 2, _2),
        (1, 3, _3),
        (1, 4, _3Max),
        (1, 5, _4Max),
        (0, 0, _5Max),
        (0, 1, _6Max),
        (0, 2, _4),
        (0, 3, _5),
        (0, 4, _6),
        (2, 0, _7Min)
    );
    ability!(e, (0, 0, _1Min), (2, 0, _2Min));
}
