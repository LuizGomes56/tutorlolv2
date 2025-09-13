use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_xerath(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min), (1, 0, _2Max, Max));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (1, 0, _1Max, Max),
        (2, 0, _2Min, Min),
        (2, 1, _3Max, Max)
    );
}
