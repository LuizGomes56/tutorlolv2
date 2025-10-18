use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_aurora(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1),
        (1, 0, _1Max),
        (1, 1, _2),
        (1, 2, _2Max),
        (1, 3, _3)
    );
    ability!(e, (0, 0, Void));
    ability!(r, (0, 0, Void));
}
