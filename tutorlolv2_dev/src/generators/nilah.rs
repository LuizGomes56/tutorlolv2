use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nilah(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Max), (0, 1, Minion1));
    ability!(e, (0, 0, _1Min));
    ability!(
        r,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (0, 2, _3Min),
        (0, 3, _4Max)
    );
}
