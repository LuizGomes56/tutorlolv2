use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_katarina(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min));
    ability!(
        r,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 2, _3Max, Max),
        (0, 3, _4Min, Min),
        (0, 4, _5Min, Min)
    );
}
