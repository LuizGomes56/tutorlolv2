use super::*;

// #![preserve] "15.20.1" | "10/14/2025"

#[tutorlolv2_macros::generator]
pub fn gen_warwick(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, Void), (0, 2, Monster));
    ability!(e, (0, 0, Void));
    ability!(r, (0, 0, Void));
}
