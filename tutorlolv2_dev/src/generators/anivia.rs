use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_anivia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void), (2, 0, _1), (2, 2, _1Max));
    ability!(e, (0, 0, Void), (0, 1, _1));
    ability!(r, (0, 0, Void), (3, 0, _1));
}
