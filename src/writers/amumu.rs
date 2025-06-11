use super::{Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage};

// #![since] "06/11/2025"
// #![patch] "25.11"
// #![pending_review] Verify if W generates the desired output

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (1, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));
}
