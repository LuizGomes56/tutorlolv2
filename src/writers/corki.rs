use super::{Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage};

// #![auto_generated]
// ! #![unstable] [X] "06/11/2025" | "25.11"

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        w,
        (1, 0, "W", Target::MINIMUM),
        (1, 1, "W_MAX", Target::MAXIMUM)
    );
    ability!(
        e,
        (0, 0, "E", Target::MINIMUM),
        (0, 1, "E_MAX", Target::MAXIMUM)
    );
    ability!(
        r,
        (1, 0, "R_0_1_0", Target::MINIMUM),
        (3, 0, "R_0_3_0", Target::MINIMUM)
    );
    merge_ability!("W");
    merge_ability!("E");
}
