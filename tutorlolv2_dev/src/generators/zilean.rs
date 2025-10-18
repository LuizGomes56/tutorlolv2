use super::*;

// #![preserve] "15.19.1" | "10/04/2025"

#[tutorlolv2_macros::generator]
pub fn gen_zilean(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, Void));
}
