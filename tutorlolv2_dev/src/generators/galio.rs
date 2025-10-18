use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_galio(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (2, 0, _3Max),
        (2, 1, Minion1)
    );
    ability!(e, (1, 0, _1Min), (1, 1, _2Min));
    ability!(r, (1, 0, _1Min));
}
