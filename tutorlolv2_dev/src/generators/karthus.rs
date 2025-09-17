use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_karthus(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min), (0, 1, Max, Min));
    ability!(e, (2, 0, Void, Min), (2, 1, Max, Min));
    ability!(r, (0, 0, Void, Min));
    merge_ability!(Q::Void, Q::Max);
    merge_ability!(E::Void, E::Max);
}
