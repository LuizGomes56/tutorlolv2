use super::{Ability, CdnChampion, Champion, FxHashMap, Target, extract_ability_damage};

// #![auto_generated]

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, "Q_0_0_1", Target::MINIMUM));
    ability!(w, (0, 0, "W_0_0_0", Target::MINIMUM));
    ability!(r, (0, 0, "R_0_0_0", Target::MINIMUM));
}
