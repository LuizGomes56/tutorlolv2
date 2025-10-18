use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_morgana(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1Max),
        (0, 1, _2Max),
        (0, 2, Minion1),
        (0, 3, _3Max)
    );
    ability!(r, (0, 1, _1Min), (0, 2, _2Max));
}
