use super::*;

// #![stable] "06/18/2025" | "25.11"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q1", Target::MINIMUM),
        (2, 0, "Q2", Target::MINIMUM),
        (2, 1, "Q_MAX", Target::MAXIMUM)
    );
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
