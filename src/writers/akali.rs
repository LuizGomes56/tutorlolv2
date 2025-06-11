use super::{
    Ability, CdnChampion, Champion, HashMap, Target, extract_ability_damage, extract_passive_damage,
};

// #![stable]
// #![since] "06/11/2025"
// #![patch] "25.11"
/// * Maximum damage from R was omitted
/// * It would add another cell to the table with value: R1 + R2_MAX
/// * E damage was split in 3 different parts, but may be merged in the future

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    passive!("P", (0, 2), Target::MINIMUM, (Some(2), None));
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        e,
        (0, 0, "E1", Target::MINIMUM),
        (2, 0, "E2", Target::MINIMUM),
        (2, 1, "E_MAX", Target::MAXIMUM)
    );
    ability!(
        r,
        (0, 0, "R1", Target::MINIMUM),
        (2, 0, "R2", Target::MINIMUM),
        (2, 1, "R2_MAX", Target::MAXIMUM)
    );
    merge_ability!("R2");
}
