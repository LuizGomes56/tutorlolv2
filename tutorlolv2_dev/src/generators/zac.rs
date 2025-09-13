use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_zac(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
    ability!(w, (0, 0, _1Min, Min), (0, 1, _2Min, Min));
    ability!(e, (2, 0, _1Min, Min));
    ability!(
        r,
        (1, 0, _1Min, Min),
        (1, 1, _2Max, Max),
        (2, 0, _3Min, Min)
    );
}
