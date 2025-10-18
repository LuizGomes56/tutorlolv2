use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_garen(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, Void));
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Min), (3, 0, Max));
    ability!(r, (0, 0, Void));
    // merge_ability!(E::Min, E::Max);
}
