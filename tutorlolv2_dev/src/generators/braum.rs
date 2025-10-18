use super::*;

// #![preserve] "15.18.1" | "09/20/2025"

#[tutorlolv2_macros::generator]
pub fn gen_braum(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(r, (1, 0, Void));
}
