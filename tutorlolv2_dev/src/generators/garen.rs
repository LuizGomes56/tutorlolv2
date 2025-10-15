use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_garen(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, Void, Min));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (0, 0, Min, Min), (3, 0, Max, Max));
    ability!(r, (0, 0, Void, Min));
    // merge_ability!(E::Min, E::Max);
}
