use super::*;

// #![preserve] "15.19.1" | "10/04/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zoe(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Max, Max), (0, 1, Minion, Min));
    ability!(w, (1, 0, Min, Min), (1, 1, Max, Max));
    ability!(e, (1, 0, Min, Min), (2, 0, _2, Min), (2, 1, Max, Max));
    merge_ability!(W::Min, W::Max);
    merge_ability!(E::Min, E::Max);
}
