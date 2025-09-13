use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nilah(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Max, Max), (0, 1, Minion1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 2, _3Min, Min),
        (0, 3, _4Max, Max)
    );
}
