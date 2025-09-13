use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_sivir(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Min, Min),
        (1, 0, Minion1, Min),
        (1, 1, _2Max, Max)
    );
    ability!(
        w,
        (0, 1, _1Min, Min),
        (0, 2, _2Min, Min),
        (0, 3, _3Min, Min),
        (0, 4, _4Min, Min)
    );
}
