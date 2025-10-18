use super::*;

// #![preserve] "15.18.1" | "09/17/2025"

#[tutorlolv2_macros::generator]
pub fn gen_kayle(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(e, (0, 0, Void), (2, 0, _1));
    ability!(r, (1, 0, Void));
}
