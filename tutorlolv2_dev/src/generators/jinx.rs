use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_jinx(data: CdnChampion) -> Champion {
    ability!(w, (0, 0, _1Min));
    ability!(e, (0, 0, _1Min));
    ability!(
        r,
        (1, 0, _1Max),
        (1, 1, Minion1),
        (2, 0, _2Max),
        (2, 1, Minion2)
    );
}
