use super::*;

// ! #![unstable] "06/19/2025" | "25.12"
// #![preserve]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q", Target::MINIMUM),
        (0, 1, "Q_MAX", Target::MINIMUM)
    );
    ability!(
        w,
        (0, 0, "W", Target::MINIMUM),
        (0, 1, "W_MAX", Target::MAXIMUM)
    );
    ability!(
        e,
        (1, 0, "E", Target::MINIMUM),
        (1, 1, "E_MAX", Target::MAXIMUM)
    );
    ability!(r, (0, 0, "R", Target::MINIMUM));
    merge_ability!("Q");
    merge_ability!("W");
    merge_ability!("E");
}
