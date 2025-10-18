use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_teemo(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max),
        (1, 0, Monster1),
        (1, 1, Monster2),
        (1, 2, Monster3)
    );
    ability!(r, (5, 0, _1Min), (5, 1, _2Max));
}
