use super::*;

// ! #![unstable] "06/19/2025" | "25.12"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 1, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R", Target::MINIMUM),
        (0, 1, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("R");
}
