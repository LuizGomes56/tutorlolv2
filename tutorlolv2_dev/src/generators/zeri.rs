use super::*;

// #![preserve] "15.20.1" | "10/14/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zeri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min, Min), (0, 1, Max, Max));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (1, 0, Min, Min), (1, 1, Max, Max));
    ability!(r, (0, 0, Void, Min));
    merge_ability!(Q::Min, Q::Max);
    merge_ability!(E::Min, E::Max);
}
