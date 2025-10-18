use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_varus(data: CdnChampion) -> Champion {
    ability!(
        q,
        (2, 0, _1Max),
        (2, 1, _2Max),
        (2, 2, Minion1),
        (2, 3, Minion2)
    );
    ability!(
        w,
        (0, 0, _1Max),
        (0, 1, Minion1),
        (1, 0, _2),
        (1, 1, _3),
        (1, 2, _4),
        (1, 3, _5),
        (4, 0, _6)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
