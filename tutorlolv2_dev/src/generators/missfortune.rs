use super::*;

// #![preserve] "10/12/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_missfortune(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(e, (0, 0, Min), (0, 1, Max));
    ability!(r, (0, 0, Max));
}
