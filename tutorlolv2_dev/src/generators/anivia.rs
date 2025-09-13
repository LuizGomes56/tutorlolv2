use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_anivia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min), (2, 0, _1, Min), (2, 2, _1Max, Max));
    ability!(e, (0, 0, Void, Min), (0, 1, _1, Min));
    ability!(r, (0, 0, Void, Min), (3, 0, _1, Min));
}
