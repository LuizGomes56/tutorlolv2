use super::*;

// #![preserve] "15.18.1" | "09/20/2025"

#[tutorlolv2_macros::generator]
pub fn gen_cassiopeia(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Min, Min), (0, 1, Max, Max));
    ability!(w, (1, 0, Min, Min), (1, 2, Max, Max));
    // Effect dependent on level
    ability!(e, (1, 0, _1, Min), (1, 3, _2Max, Max));
    ability!(r, (0, 0, Void, Min));
}
