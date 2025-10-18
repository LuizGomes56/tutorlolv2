use super::*;

// #![preserve] "15.20.1" | "10/16/2025"

#[tutorlolv2_macros::generator]
pub fn gen_malphite(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(w, (0, 0, _1), (1, 0, _2));
    ability!(e, (0, 1, Void));
    ability!(r, (0, 0, Void));
}
