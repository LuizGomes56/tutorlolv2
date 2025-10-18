use super::*;

// #![preserve] "15.20.1" | "10/14/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zeri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min), (0, 1, Max));
    ability!(w, (0, 0, Void));
    ability!(e, (1, 0, Min), (1, 1, Max));
    ability!(r, (0, 0, Void));
}
