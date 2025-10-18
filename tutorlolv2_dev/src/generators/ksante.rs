use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ksante(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        w,
        (1, 0, _1),
        (1, 1, _2),
        (1, 2, _3Max),
        (4, 0, Monster1),
        (4, 1, _4Min)
    );
    ability!(
        r,
        (0, 0, _1Min),
        (3, 0, _2Min),
        (3, 1, _3Max)
    );
}
