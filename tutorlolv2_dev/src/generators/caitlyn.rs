use super::*;

// #![preserve] "15.18.1" | "09/20/2025"

#[tutorlolv2_macros::generator]
pub fn gen_caitlyn(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Max), (0, 1, Min));
    ability!(w, (2, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(r, (2, 0, Void));
}
