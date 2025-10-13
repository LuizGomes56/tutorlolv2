use super::*;

// #![preserve] "10/12/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_missfortune(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(e, (0, 0, Min, Min), (0, 1, Max, Max));
    ability!(r, (0, 0, Max, Max));
    merge_ability!(E::Min, E::Max);
}
