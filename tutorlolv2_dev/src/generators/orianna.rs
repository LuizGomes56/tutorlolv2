use super::*;

// #![preserve] "10/13/2025" | "25.20"

#[tutorlolv2_macros::generator]
pub fn gen_orianna(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min), (0, 1, Max));
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(r, (0, 0, Void));
}
