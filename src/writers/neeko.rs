use super::{Ability, CdnChampion, Champion, FxHashMap, Target, extract_ability_damage};

// #![stable] "06/18/2025" | "25.11"
// #![unsupported] MINION | MONSTER
/// * Column Cell Template Exibition = `{LEFT} - {RIGHT}`
/// * Q `{LEFT}` represents initial damage, and `{RIGHT}` the total.
/// * Q1 represents the damage of each secondary explosion

#[writer_macros::writer]
pub fn transform(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, "Q", Target::MINIMUM),
        (1, 0, "Q1", Target::MINIMUM),
        (1, 1, "Q_MAX", Target::MAXIMUM),
        (2, 0, "Q_MONSTER_BONUS", Target::MINIMUM),
    );
    ability!(w, (0, 0, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(r, (2, 0, "R", Target::MINIMUM));
    merge_ability!("Q");
}
