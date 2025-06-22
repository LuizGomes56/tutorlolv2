use super::*;

// #![stable] "06/11/2025" | "25.11"
// * R is captured by the generator but does not deal any damage
// * Damage reductions are not planned to ever be implemented

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E", Target::MINIMUM),
        (0, 1, "E_MAX", Target::MAXIMUM)
    );
    merge_ability!("E");
}
