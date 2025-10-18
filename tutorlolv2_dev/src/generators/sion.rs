use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sion(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max),
        (0, 1, _2Max),
        (0, 2, Minion1),
        (3, 0, _3Max),
        (3, 1, Monster1),
        (3, 2, Minion2),
        (3, 3, Monster2)
    );
    ability!(w, (3, 0, _1Min));
    ability!(e, (0, 0, _1Min));
    ability!(r, (2, 0, _1Max), (2, 1, Minion1));
}
