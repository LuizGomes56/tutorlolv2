use super::{Ability, CdnChampion, Champion, FxHashMap, Target, extract_ability_damage};

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
    ability!(w, (1, 0, "W", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
}
