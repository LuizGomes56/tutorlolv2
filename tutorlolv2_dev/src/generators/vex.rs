use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_vex(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (0, 0, Void, Min));
    ability!(e, (0, 0, Void, Min));
    ability!(r, (0, 0, Minion, Min), (2, 0, Void, Min), (2, 1, Max, Max));
}
