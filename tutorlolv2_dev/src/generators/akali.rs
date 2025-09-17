use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_akali(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(e, (0, 0, Void, Min), (2, 0, _1, Min), (2, 1, _1Max, Max));
    merge_ability!(E::_1, E::_1Max);
    ability!(r, (0, 0, Void, Min), (2, 0, _1Max, Max), (2, 1, _1Min, Min));
    merge_ability!(R::_1Min, R::_1Max);
}
