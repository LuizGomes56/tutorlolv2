use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_galio(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(
        w,
        (0, 0, _1Min, Min),
        (0, 1, _2Min, Min),
        (2, 0, _3Max, Max),
        (2, 1, Minion1, Min)
    );
    ability!(e, (1, 0, _1Min, Min), (1, 1, _2Min, Min));
    ability!(r, (1, 0, _1Min, Min));
}
