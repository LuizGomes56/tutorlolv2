use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_belveth(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (2, 0, _2Min),
        (2, 1, Monster1),
        (2, 2, Monster2)
    );
    ability!(w, (0, 0, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (3, 0, Monster1),
        (3, 1, Monster2),
        (5, 0, _2Max),
        (5, 1, Minion1)
    );
    ability!(
        r,
        (1, 0, _1Min),
        (2, 0, _2),
        (2, 1, Monster1)
    );
}
