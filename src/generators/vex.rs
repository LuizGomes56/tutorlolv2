use super::*;

// #![stable] "08/07/2025" | "25.15"

#[generator_macros::generator]
pub fn gen_vex(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Min));
    ability!(w, (0, 0, "W", Min));
    ability!(e, (0, 0, "E", Min));
    ability!(
        r,
        (0, 0, "R_MINION", Min),
        (2, 0, "R", Min),
        (2, 1, "R_MAX", Max)
    );
    merge_ability!("R");
}
