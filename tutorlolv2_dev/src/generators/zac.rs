use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_zac(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (0, 1, _2Max));
    ability!(w, (0, 0, _1Min), (0, 1, _2Min));
    ability!(e, (2, 0, _1Min));
    ability!(
        r,
        (1, 0, _1Min),
        (1, 1, _2Max),
        (2, 0, _3Min)
    );
}
