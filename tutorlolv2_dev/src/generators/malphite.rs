use super::*;

// #![preserve] "15.20.1" | "10/16/2025"

#[tutorlolv2_macros::generator]
pub fn gen_malphite(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (0, 0, _1, Min), (1, 0, _2, Min));
    ability!(e, (0, 1, Void, Min));
    ability!(r, (0, 0, Void, Min));
}
