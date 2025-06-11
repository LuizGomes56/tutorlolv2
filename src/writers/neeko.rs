use super::{Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage};

// #![stable]
// #![since] "06/01/2025"
// #![patch] "25.9"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q1", Target::MINIMUM),
        (1, 0, "Q2", Target::MINIMUM),
        (1, 1, "Q_MAX", Target::MINIMUM),
        (2, 0, "Q_MONSTER_BONUS", Target::MINIMUM),
    );
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(r, (2, 0, "R", Target::MINIMUM));
}
