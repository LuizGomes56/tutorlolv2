use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kled(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1),
        (0, 1, _1Min),
        (2, 0, _2Min),
        (2, 2, _3Max),
        (0, 0, _4Min),
        (2, 0, _5Max),
        (2, 1, _6Min)
    );
    ability!(w, (0, 0, _1), (2, 0, _2));
    ability!(e, (0, 0, _1Min), (2, 0, _2Max));
    ability!(r, (1, 0, _1Max), (1, 1, Minion1));
}
