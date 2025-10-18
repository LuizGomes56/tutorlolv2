use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yunara(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1),
        (0, 1, _2Max),
        (0, 3, _3),
        (0, 4, _4Max),
        (2, 0, _5)
    );
    ability!(
        w,
        (0, 0, _1Min),
        (0, 1, _2Min),
        (0, 2, _3Max)
    );
    ability!(r, (1, 0, _1Min));
}
