use super::*;

// #![preserve] "15.19.1" | "10/04/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zoe(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Max), (0, 1, Minion));
    ability!(w, (1, 0, Min), (1, 1, Max));
    ability!(e, (1, 0, Min), (2, 0, _2), (2, 1, Max));
}
