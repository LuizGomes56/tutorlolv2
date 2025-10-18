use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ornn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (1, 0, Minion1),
        (1, 1, Monster1),
        (1, 2, _1Max),
        (1, 3, Monster2),
        (2, 0, _2Min),
        (2, 1, _3Max)
    );
    ability!(e, (0, 0, _1Min));
    ability!(r, (0, 0, _1Min), (3, 0, _2Max));
}
