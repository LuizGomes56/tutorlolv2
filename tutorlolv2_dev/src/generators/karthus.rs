use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_karthus(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void), (0, 1, Max));
    ability!(e, (2, 0, Void), (2, 1, Max));
    ability!(r, (0, 0, Void));
}
