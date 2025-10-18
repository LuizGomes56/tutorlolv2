use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_akshan(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void), (1, 0, _1), (3, 0, _1Max));
    ability!(e, (4, 0, Void));
    ability!(
        r,
        (4, 0, _1),
        (4, 1, _1Max),
        (4, 2, _2),
        (4, 3, _2Max)
    );
}
