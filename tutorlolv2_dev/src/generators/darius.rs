use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_darius(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min), (0, 1, Max));
    ability!(w, (0, 0, Void));
    ability!(r, (0, 0, _1), (0, 1, _1Max), (0, 2, _2));
}
