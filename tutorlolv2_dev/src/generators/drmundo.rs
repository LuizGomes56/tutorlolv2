use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_drmundo(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (2, 0, Monster1, Min),
        (2, 1, Minion1, Min)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (2, 0, _3Min, Min)
    );
    ability!(e, (0, 0, _1, Min), (0, 1, _2, Min), (3, 0, _3, Min));
}
