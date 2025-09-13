use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_draven(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (4, 0, Minion1, Min),
        (4, 1, _3Max, Max)
    );
}
