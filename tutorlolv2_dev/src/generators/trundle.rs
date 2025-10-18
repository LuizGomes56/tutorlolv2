use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_trundle(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1), (1, 0, _2Min), (1, 1, _3));
    ability!(
        r,
        (0, 0, _1Max),
        (1, 0, _2Min),
        (1, 1, _3Min)
    );
}
