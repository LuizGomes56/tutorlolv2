use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_viego(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1),
        (0, 1, _1Max),
        (0, 2, Monster2),
        (0, 3, Minion1),
        (3, 0, _2),
        (3, 1, _3)
    );
    ability!(w, (1, 0, _1Min));
    ability!(r, (0, 0, _1Min));
}
