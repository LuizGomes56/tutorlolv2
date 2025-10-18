use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sivir(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min),
        (1, 0, Minion1),
        (1, 1, _2Max)
    );
    ability!(
        w,
        (0, 1, _1Min),
        (0, 2, _2Min),
        (0, 3, _3Min),
        (0, 4, _4Min)
    );
}
