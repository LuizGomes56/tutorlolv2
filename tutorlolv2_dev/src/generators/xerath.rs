use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_xerath(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min));
    ability!(w, (0, 0, _1Min), (1, 0, _2Max));
    ability!(e, (0, 0, _1Min));
    ability!(
        r,
        (1, 0, _1Max),
        (2, 0, _2Min),
        (2, 1, _3Max)
    );
}
