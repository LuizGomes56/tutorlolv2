use super::{Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage};

// #![stable] "06/11/2025" | "25.11"
// #![unsupported] [P] BARD_STACKS (Meeps)

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
}
