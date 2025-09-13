use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_gragas(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, _1Max, Max),
        (1, 1, _2Max, Max),
        (1, 3, Minion1, Min),
        (1, 4, Minion2, Min)
    );
    ability!(
        w,
        (0, 0, _1Min, Min),
        (1, 0, _2, Min),
        (1, 1, Monster1, Min)
    );
    ability!(e, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min));
}
