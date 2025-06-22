use super::*;

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(
        e,
        (1, 0, "E", Target::MINIMUM),
        (1, 1, "E_MAX", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 0, "R", Target::MINIMUM),
        (1, 0, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("E");
    merge_ability!("R");
}
