use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E", Target::MINIMUM),
        (0, 1, "E_MAX", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 0, "R_0_0_0", Target::MINIMUM),
        (0, 1, "R_0_0_1", Target::MINIMUM)
    );
    merge_ability!("E");
}
