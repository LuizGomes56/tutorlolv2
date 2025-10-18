use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yasuo(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (2, 0, _2),
        (2, 1, _3),
        (2, 2, _4Max)
    );
    ability!(r, (3, 0, _1Min));
}
