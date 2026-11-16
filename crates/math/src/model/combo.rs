//! Steps of a combo and the columns its evaluation writes, see [`crate::combo`]
use super::*;

/// One step of a combo, as the client sends it. Steps are evaluated in order
/// against the target's health of that moment, see [`crate::combo`]
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize,
)]
pub enum ComboStep {
    Ability(AbilityId),
    /// Basic attack. Owned every-hit items apply on it automatically
    Attack,
    /// Critical strike, a basic attack too
    Crit,
    /// Damage of one owned item, as Lich Bane's proc
    Item(ItemId),
    /// Damage of one owned rune
    Rune(RuneId),
    /// A basic attack with the owned every-hit effects
    OnhitMin,
    /// A basic attack with every owned on-hit effect, procs included
    OnhitMax,
    Ignite,
}

impl From<types::ComboElement> for ComboStep {
    fn from(element: types::ComboElement) -> Self {
        match element {
            types::ComboElement::Ability(ability_id) => Self::Ability(ability_id),
            types::ComboElement::Attack => Self::Attack,
        }
    }
}

/// Combo columns of one row of a [`DamageTable`]: the damage of each expanded
/// step, then [`Self::SUMMARY`] values
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboRow<'a> {
    /// Damage of each step, in the order of `Realtime::combo`
    pub steps: &'a [i32],
    /// Sum of every step. It may exceed the starting health (overkill)
    pub total: i32,
    /// Health after the last step, negative after an overkill
    pub final_health: i32,
    /// Index of the step that brings the health to `0`, if any
    pub kill_step: Option<usize>,
}

impl<'a> ComboRow<'a> {
    /// Columns after the steps: total, final health and killing step
    pub const SUMMARY: usize = 3;

    /// Reads the combo columns written by [`crate::combo`], `None` if empty
    pub const fn new(columns: &'a [i32]) -> Option<Self> {
        let Some((steps, summary)) = columns.split_last_chunk::<3>() else {
            return None;
        };

        let [total, final_health, kill_step] = *summary;

        Some(Self {
            steps,
            total,
            final_health,
            kill_step: match kill_step {
                ..0 => None,
                step => Some(step as usize),
            },
        })
    }
}
