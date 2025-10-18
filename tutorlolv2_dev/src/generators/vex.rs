use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_vex(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Void));
    ability!(r, (0, 0, Minion), (2, 0, Void), (2, 1, Max));
}
