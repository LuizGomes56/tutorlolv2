use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_yuumi(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min), (1, 1, _2Max), (2, 0, _3));
    ability!(
        r,
        (4, 0, _1Min),
        (4, 1, _2Min),
        (4, 2, _3Max)
    );
}
