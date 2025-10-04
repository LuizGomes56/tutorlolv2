use super::*;

// #![preserve] "15.19.1" | "10/03/2025"

#[tutorlolv2_macros::generator]
pub fn gen_fiora(data: CdnChampion) -> Champion {
    ability!(q, (2, 0, Void, Min));
    ability!(w, (1, 0, Void, Min));
    ability!(e, (2, 0, Void, Min));
}
