use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_gnar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min, Min), (0, 1, Max, Max), (0, 0, Mega, Min));
    ability!(w, (2, 0, Void, Min), (0, 0, Mega, Min));
    ability!(e, (4, 0, Void, Min), (0, 0, Mega, Min));
    ability!(r, (0, 0, Max, Max), (1, 1, Min, Min));
    // merge_ability!(Q::Min, Q::Max);
    // merge_ability!(R::Min, R::Max);
}
