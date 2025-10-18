use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_briar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(w, (2, 2, _1Min), (0, 0, _2), (1, 0, _3));
    ability!(
        e,
        (2, 0, _1),
        (2, 1, _2Max),
        (3, 0, _3Max),
        (3, 1, Minion1)
    );
    ability!(r, (3, 0, _1Min));
}
