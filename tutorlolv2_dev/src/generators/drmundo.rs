use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_drmundo(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (2, 0, Monster1),
        (2, 1, Minion1)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (2, 0, _3Min)
    );
    ability!(e, (0, 0, _1), (0, 1, _2), (3, 0, _3));
}
