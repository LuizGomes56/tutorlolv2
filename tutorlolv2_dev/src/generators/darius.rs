use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_darius(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min, Min), (0, 1, Max, Max));
    ability!(w, (0, 0, Void, Min));
    ability!(r, (0, 0, _1, Min), (0, 1, _1Max, Max), (0, 2, _2, Min));
}
