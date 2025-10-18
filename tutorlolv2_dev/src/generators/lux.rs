use super::*;

// #![preserve] "15.20.1" | "10/16/2025"

#[tutorlolv2_macros::generator]
pub fn gen_lux(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(e, (2, 0, Void));
    ability!(r, (0, 0, Void));
}
