use super::{Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, "Q", Target::MINIMUM));
}
