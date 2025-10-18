use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kayn(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (1, 0, _3Min),
        (1, 1, _4Max),
        (2, 0, Monster1),
        (2, 1, Monster2)
    );
    ability!(w, (0, 0, _1Min));
    ability!(r, (3, 0, _1Min));
}
