use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_masteryi(data: CdnChampion) -> Champion {
    ability!(
        q,
        (3, 0, Monster1),
        (3, 1, Monster2),
        (3, 2, _1Max),
        (3, 3, Monster3),
        (3, 4, _2Min),
        (3, 5, _3Min),
        (3, 6, Monster4)
    );
    ability!(w, (2, 0, _1Min), (2, 1, _2Min));
    ability!(e, (0, 0, _1));
}
