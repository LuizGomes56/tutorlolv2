use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_shen(data: CdnChampion) -> Champion {
    ability!(
        q,
        (1, 0, Monster1),
        (2, 0, _1),
        (2, 1, _2Max),
        (3, 0, _3),
        (3, 1, _4Max)
    );
    ability!(e, (0, 0, _1Min));
}
