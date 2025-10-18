use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_poppy(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1),
        (0, 1, Monster2),
        (0, 2, _1Min),
        (1, 1, Monster3),
        (1, 2, _2Max)
    );
    ability!(w, (0, 0, _1Min));
    ability!(e, (0, 0, _1Min), (1, 1, _2Max));
    ability!(r, (1, 0, _1Max), (3, 0, _2Min));
}
