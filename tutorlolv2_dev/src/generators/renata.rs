use super::*;

// #![preserve] "10/13/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_renata(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(e, (1, 0, Void));
}
