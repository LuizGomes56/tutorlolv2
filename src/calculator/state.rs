//! Calculator inputs: the web app's `calculator/mod.rs`, `exceptions.rs` and `utils/tray.rs`.
//!
//! The web wraps these in `Rc` so yew can compare snapshots cheaply. Here the calculator view
//! owns them and mutates them in place, so they are plain values.
use crate::random::random_variant;
use std::{
    marker::PhantomData,
    num::NonZeroU8,
    sync::atomic::{AtomicU64, Ordering},
};
use tutorlolv2::{
    ChampionId, ItemId, RuneId, ValueId,
    bitset::{BitSet, BitSetExc},
    model::{AbilityLevels, EnemyStats, PlayerStats, ValueException},
};

/// Values with stable identities, so one of two equal entries can be removed.
#[derive(Clone, Debug)]
pub struct Tray<T: Copy> {
    ids: Vec<u64>,
    values: Vec<T>,
}

impl<T: Copy> Default for Tray<T> {
    fn default() -> Self {
        Self {
            ids: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<T: Copy> Tray<T> {
    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, value: T) {
        // The web uses random ids. Unique ones are enough.
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        self.ids.push(NEXT_ID.fetch_add(1, Ordering::Relaxed));
        self.values.push(value);
    }

    /// Each value with its id, in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (u64, T)> + '_ {
        self.ids.iter().copied().zip(self.values.iter().copied())
    }

    pub fn remove_by_id(&mut self, id: u64) {
        if let Some(index) = self.ids.iter().position(|&other| other == id) {
            self.ids.remove(index);
            self.values.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.values.clear();
    }

    /// Keeps the values for which `keep` is true, with their ids.
    pub fn retain(&mut self, mut keep: impl FnMut(T) -> bool) {
        let mut index = 0;

        while index < self.values.len() {
            if keep(self.values[index]) {
                index += 1;
            } else {
                self.ids.remove(index);
                self.values.remove(index);
            }
        }
    }
}

impl<T: Copy> FromIterator<T> for Tray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tray = Self::default();

        for value in iter {
            tray.push(value);
        }

        tray
    }
}

/// Packed exceptions in ascending id order. `T` supplies the item or rune domain, which
/// [`ValueException`] itself does not encode.
#[derive(Clone, Debug)]
pub struct ExceptionMap<T: ValueId> {
    keys: BitSet,
    values: Vec<ValueException>,
    marker: PhantomData<T>,
}

impl<T: ValueId> Default for ExceptionMap<T> {
    fn default() -> Self {
        Self {
            keys: BitSet::EMPTY,
            values: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl<T: ValueId> ExceptionMap<T> {
    pub fn values(&self) -> &[ValueException] {
        &self.values
    }

    pub fn get(&self, key: &T) -> Option<&ValueException> {
        let index = key.index() as u32;

        self.keys.contains_const(index).then(|| {
            let position = self.keys.count_lesser_elements_const(index) as usize;
            &self.values[position]
        })
    }

    pub fn insert(&mut self, key: T, stacks: u32) {
        let index = key.index() as u32;

        // The rank of the id is also its offset into the packed values.
        let position = self.keys.count_lesser_elements_const(index) as usize;
        let value = key.pack_exc(stacks);

        if self.keys.contains_const(index) {
            self.values[position] = value;
        } else {
            self.values.insert(position, value);
            self.keys.insert_const(index);
        }
    }

    /// Resets the stacks of `value` if it is an exception, then calls `insert`.
    pub fn push_item(&mut self, value: T, filter: BitSetExc, insert: impl FnOnce(T)) {
        if filter.contains_const(value.index() as _) {
            self.insert(value, 0);
        }

        insert(value);
    }
}

/// Exceptions (items whose damage depends on stacks) for allies and enemies.
pub fn item_exceptions(ally: bool) -> BitSetExc {
    ItemId::exceptions(ally)
}

/// Minimum data of a player: the web's `PlayerData<T>`.
#[derive(Clone, Debug)]
pub struct PlayerData<T> {
    pub stats: T,
    pub items: Tray<ItemId>,
    pub item_exceptions: ExceptionMap<ItemId>,
    pub stacks: u32,
    pub level: NonZeroU8,
    pub champion_id: ChampionId,
    pub is_mega_gnar: bool,
    pub infer_stats: bool,
}

impl<T> PlayerData<T> {
    /// Adds an item. Items with stacks start at zero stacks, which shows their input.
    pub fn insert_item(&mut self, item: ItemId, ally: bool) {
        let items = &mut self.items;

        self.item_exceptions
            .push_item(item, item_exceptions(ally), |item| items.push(item));
    }

    /// Replaces the items, as a click on a position's recommendations does.
    pub fn replace_items(&mut self, items: &[ItemId], ally: bool) {
        self.items.clear();

        for &item in items {
            self.insert_item(item, ally);
        }
    }
}

impl<T: Default> PlayerData<T> {
    /// A random champion with the recommended items of its main position.
    pub fn random(ally: bool) -> Self {
        Self::new(random_variant(), ally)
    }

    pub fn new(champion_id: ChampionId, ally: bool) -> Self {
        let mut data = Self {
            stats: T::default(),
            items: Tray::default(),
            item_exceptions: ExceptionMap::default(),
            stacks: 0,
            level: const { NonZeroU8::new(18).unwrap() },
            champion_id,
            is_mega_gnar: false,
            infer_stats: true,
        };

        data.replace_items(
            champion_id.recommended_items(champion_id.main_position()),
            ally,
        );
        data
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub runes: Tray<RuneId>,
    pub rune_exceptions: ExceptionMap<RuneId>,
    pub abilities: AbilityLevels,
    /// Current health sent while the stats are inferred. Zero means full health.
    pub current_health: f32,
    pub data: PlayerData<PlayerStats>,
}

impl Player {
    pub fn random() -> Self {
        let data = PlayerData::random(true);
        let champion_id = data.champion_id;

        Self {
            runes: champion_id
                .recommended_runes(champion_id.main_position())
                .iter()
                .copied()
                .collect(),
            rune_exceptions: ExceptionMap::default(),
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            current_health: 0.0,
            data,
        }
    }
}

pub type Enemy = PlayerData<EnemyStats>;

/// `Enemies::MAX_ENEMIES`.
pub const MAX_ENEMIES: usize = 5;

/// Three random enemies, as on the web.
pub fn default_enemies() -> Vec<Enemy> {
    (0..3).map(|_| Enemy::random(false)).collect()
}

/// Editable stats: the web's `PlayerStatsField`, with each field's input accessors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StatField {
    AbilityPower,
    AttackDamage,
    MaxHealth,
    CurrentHealth,
    Armor,
    ArmorPenetrationFlat,
    ArmorPenetrationPercent,
    MagicResist,
    MagicPenetrationFlat,
    MagicPenetrationPercent,
    CritChance,
    CritDamage,
    MaxMana,
    CurrentMana,
    AttackSpeed,
}

impl StatField {
    /// `impl_stat_display!(PlayerStats { .. })`, in display order.
    pub const PLAYER: [Self; 15] = [
        Self::AbilityPower,
        Self::AttackDamage,
        Self::MaxHealth,
        Self::CurrentHealth,
        Self::Armor,
        Self::ArmorPenetrationFlat,
        Self::ArmorPenetrationPercent,
        Self::MagicResist,
        Self::MagicPenetrationFlat,
        Self::MagicPenetrationPercent,
        Self::CritChance,
        Self::CritDamage,
        Self::MaxMana,
        Self::CurrentMana,
        Self::AttackSpeed,
    ];

    /// `impl_stat_display!(EnemyStats { .. })`.
    pub const ENEMY: [Self; 4] = [
        Self::MaxHealth,
        Self::CurrentHealth,
        Self::Armor,
        Self::MagicResist,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::AbilityPower => "Ability Power",
            Self::Armor => "Armor",
            Self::ArmorPenetrationFlat => "Armor Pen. Flat",
            Self::ArmorPenetrationPercent => "Armor Pen. %",
            Self::AttackDamage => "Attack Damage",
            Self::AttackSpeed => "Attack Speed",
            Self::CritChance => "Crit Chance",
            Self::CritDamage => "Crit Damage",
            Self::CurrentHealth => "Current Health",
            Self::MagicPenetrationFlat => "Magic Pen. Flat",
            Self::MagicPenetrationPercent => "Magic Pen. %",
            Self::MagicResist => "Magic Resist",
            Self::MaxHealth => "Max Health",
            Self::MaxMana => "Max Mana",
            Self::CurrentMana => "Current Mana",
        }
    }

    /// `ImageType::Stats`.
    pub const fn icon(self) -> &'static str {
        match self {
            Self::AbilityPower => "stats/ability_power.svg",
            Self::Armor => "stats/armor.svg",
            Self::ArmorPenetrationFlat | Self::ArmorPenetrationPercent => {
                "stats/armor_penetration.svg"
            }
            Self::AttackDamage => "stats/attack_damage.svg",
            Self::AttackSpeed => "stats/attack_speed.svg",
            Self::CritChance => "stats/crit_chance.svg",
            Self::CritDamage => "stats/crit_damage.svg",
            Self::CurrentHealth | Self::MaxHealth => "stats/health.svg",
            Self::CurrentMana | Self::MaxMana => "stats/mana.svg",
            Self::MagicPenetrationFlat | Self::MagicPenetrationPercent => {
                "stats/magic_penetration.svg"
            }
            Self::MagicResist => "stats/magic_resist.svg",
        }
    }

    /// `StatCell` shows attack speed with two decimals and the rest as integers.
    pub fn format(self, value: f32) -> String {
        match self {
            Self::AttackSpeed => format!("{value:.2}"),
            _ => (value as i32).to_string(),
        }
    }
}

/// Stats that can be read and written by [`StatField`].
pub trait Stats {
    fn get(&self, field: StatField) -> f32;
    fn set(&mut self, field: StatField, value: f32);
}

impl Stats for PlayerStats {
    fn get(&self, field: StatField) -> f32 {
        match field {
            StatField::AbilityPower => self.ability_power,
            StatField::Armor => self.armor,
            StatField::ArmorPenetrationFlat => self.armor_penetration_flat,
            StatField::ArmorPenetrationPercent => self.armor_penetration_percent,
            StatField::AttackDamage => self.attack_damage,
            StatField::AttackSpeed => self.attack_speed,
            StatField::CritChance => self.crit_chance,
            StatField::CritDamage => self.crit_damage,
            StatField::CurrentHealth => self.current_health,
            StatField::MagicPenetrationFlat => self.magic_penetration_flat,
            StatField::MagicPenetrationPercent => self.magic_penetration_percent,
            StatField::MagicResist => self.magic_resist,
            StatField::MaxHealth => self.max_health,
            StatField::MaxMana => self.max_mana,
            StatField::CurrentMana => self.current_mana,
        }
    }

    fn set(&mut self, field: StatField, value: f32) {
        let stat = match field {
            StatField::AbilityPower => &mut self.ability_power,
            StatField::Armor => &mut self.armor,
            StatField::ArmorPenetrationFlat => &mut self.armor_penetration_flat,
            StatField::ArmorPenetrationPercent => &mut self.armor_penetration_percent,
            StatField::AttackDamage => &mut self.attack_damage,
            StatField::AttackSpeed => &mut self.attack_speed,
            StatField::CritChance => &mut self.crit_chance,
            StatField::CritDamage => &mut self.crit_damage,
            StatField::CurrentHealth => &mut self.current_health,
            StatField::MagicPenetrationFlat => &mut self.magic_penetration_flat,
            StatField::MagicPenetrationPercent => &mut self.magic_penetration_percent,
            StatField::MagicResist => &mut self.magic_resist,
            StatField::MaxHealth => &mut self.max_health,
            StatField::MaxMana => &mut self.max_mana,
            StatField::CurrentMana => &mut self.current_mana,
        };

        *stat = value;
    }
}

impl Stats for EnemyStats {
    fn get(&self, field: StatField) -> f32 {
        match field {
            StatField::Armor => self.armor,
            StatField::CurrentHealth => self.current_health,
            StatField::MagicResist => self.magic_resist,
            StatField::MaxHealth => self.max_health,
            _ => 0.0,
        }
    }

    fn set(&mut self, field: StatField, value: f32) {
        match field {
            StatField::Armor => self.armor = value,
            StatField::CurrentHealth => self.current_health = value,
            StatField::MagicResist => self.magic_resist = value,
            StatField::MaxHealth => self.max_health = value,
            _ => {}
        }
    }
}
