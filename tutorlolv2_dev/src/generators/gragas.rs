use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gragas(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Max),
        (1, 1, _2Max),
        (1, 3, Minion1),
        (1, 4, Minion2)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (1, 0, _2),
        (1, 1, Monster1)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
