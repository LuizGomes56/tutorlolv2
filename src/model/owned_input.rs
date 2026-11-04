//! Owned calculator requests and safe views over their collections.
use super::*;

#[derive(Clone, Debug, Default, Encode, Decode, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OwnedInputGame {
    pub active_player: OwnedInputActivePlayer,
    pub enemy_players: Box<[OwnedInputMinData<EnemyStats>]>,
    pub dragons: Dragons,
}

#[derive(Clone, Debug, Default, Encode, Decode, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OwnedInputActivePlayer {
    pub runes: Box<[RuneId]>,
    pub rune_exceptions: Box<[ValueException]>,
    pub abilities: AbilityLevels,
    pub data: OwnedInputMinData<PlayerStats>,
}

#[derive(Clone, Debug, Encode, Decode, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct OwnedInputMinData<T> {
    pub stats: Option<T>,
    pub items: Box<[ItemId]>,
    pub item_exceptions: Box<[ValueException]>,
    pub stacks: u32,
    pub level: NonZeroU8,
    pub champion_id: ChampionId,
    pub is_mega_gnar: bool,
}

impl<T> Default for OwnedInputMinData<T> {
    fn default() -> Self {
        Self {
            stats: None,
            items: Box::new([]),
            item_exceptions: Box::new([]),
            stacks: 0,
            level: NonZeroU8::MIN,
            champion_id: ChampionId::default(),
            is_mega_gnar: false,
        }
    }
}

impl<T: Copy> OwnedInputMinData<T> {
    pub fn as_input(&self) -> InputMinData<'_, T> {
        InputMinData {
            stats: self.stats,
            items: &self.items,
            item_exceptions: &self.item_exceptions,
            stacks: self.stacks,
            level: self.level,
            champion_id: self.champion_id,
            is_mega_gnar: self.is_mega_gnar,
        }
    }
}

impl<'a, T: Copy> From<&'a OwnedInputMinData<T>> for InputMinData<'a, T> {
    fn from(value: &'a OwnedInputMinData<T>) -> Self {
        value.as_input()
    }
}

impl OwnedInputActivePlayer {
    pub fn as_input(&self) -> InputActivePlayer<'_> {
        InputActivePlayer {
            runes: &self.runes,
            rune_exceptions: &self.rune_exceptions,
            abilities: self.abilities,
            data: self.data.as_input(),
        }
    }
}

impl<'a> From<&'a OwnedInputActivePlayer> for InputActivePlayer<'a> {
    fn from(value: &'a OwnedInputActivePlayer) -> Self {
        value.as_input()
    }
}

/// Owns only the enemy view records; their collections borrow the request.
/// Keep this value alive while passing `as_input()` to the calculator.
#[derive(Debug)]
pub struct PreparedInputGame<'a> {
    active_player: InputActivePlayer<'a>,
    enemy_players: Box<[InputMinData<'a, EnemyStats>]>,
    dragons: Dragons,
}

impl OwnedInputGame {
    pub fn prepare(&self) -> PreparedInputGame<'_> {
        PreparedInputGame {
            active_player: self.active_player.as_input(),
            enemy_players: self
                .enemy_players
                .iter()
                .map(OwnedInputMinData::as_input)
                .collect(),
            dragons: self.dragons,
        }
    }
}

impl PreparedInputGame<'_> {
    pub fn as_input(&self) -> InputGame<'_> {
        InputGame {
            active_player: self.active_player,
            enemy_players: &self.enemy_players,
            dragons: self.dragons,
        }
    }
}
