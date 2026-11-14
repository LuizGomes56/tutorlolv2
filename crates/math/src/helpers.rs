//! This module exports several helper functions that help get
//! base stats, bonus stats, damaging abilities, damage against
//! jungle monsters, turrets, other champions, etc.
//!
//! These are functions that are used internally in the functions
//! [`crate::calculator`] and [`crate::realtime`], but if you want
//! to customize the damage calculation process, this is the place
//! to start
//!
//! Note that most functions have the `const` qualifier, which you
//! should use to debug and check if the calculated values are correct
//! without having to compile your project.
//!
//! Function pointers can't be called in const context, so if you still
//! want to do it, you can use the [`crate::const_eval`] module to
//! do it

use {
    crate::{
        AbilityBatch, ChampionId, ItemId, Monster, RuneId, WikiModifiers,
        bitset::BitSet,
        libgen::{Champion, Closure, L_TWRD, Stat, WikiStats, champions, items},
        model::*,
    },
    alloc::boxed::Box,
    const_sized_bit_set::bit_set_array::BitSetArray,
    core::{mem::MaybeUninit, num::NonZeroU8},
    strum::{EnumCount, VariantArray},
    types::{
        AbilityId, AdaptiveType, AttackSpeedRatio, AttackType, Ctx, CtxAttacker, CtxFlags,
        CtxTarget, DamageType, GameMap, MapMode, StatName, TypeMetadata,
    },
};

pub const fn has_item<const N: usize>(origin: &BitSet, check_for: [ItemId; N]) -> bool {
    let mut i = 0;

    while i < N {
        if origin.contains_const(check_for[i] as _) {
            return true;
        }

        i += 1;
    }

    false
}

impl Modifiers {
    pub const fn apply_item(&mut self, item: ItemId) {
        match item {
            ItemId::Riftmaker => self.damages.global_mod *= RiotFormulas::RIFTMAKER_BONUS_DAMAGE,
            ItemId::Shadowflame => {
                self.damages.magic_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
                self.damages.true_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
            }
            ItemId::SpearOfShojin => {
                self.abilities.q *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                self.abilities.w *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                self.abilities.e *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                self.abilities.r *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
            }
            _ => {}
        }
    }

    pub const fn apply_rune(&mut self, rune: RuneId, stats: &PlayerStats) {
        match rune {
            RuneId::AxiomArcanist => self.abilities.r *= RiotFormulas::AXIOM_ARCANIST_BONUS_DAMAGE,
            RuneId::LastStand => {
                self.damages.global_mod *=
                    RiotFormulas::get_last_stand(stats.missing_health_ratio())
            }
            _ => {}
        }
    }

    pub const fn apply_loadout(&mut self, items: &[ItemId], runes: &[RuneId], stats: &PlayerStats) {
        let mut i = 0;

        while i < items.len() {
            self.apply_item(items[i]);
            i += 1;
        }

        i = 0;

        while i < runes.len() {
            self.apply_rune(runes[i], stats);
            i += 1;
        }
    }

    /// Apply target defenses after attacker-side buffs have been collected.
    pub const fn against(mut self, enemy: &EnemyFullState, adaptive_type: AdaptiveType) -> Self {
        self.damages.adaptive_type = adaptive_type;
        self.damages.physical_mod *= enemy.armor_values.modifier * enemy.modifiers.physical_mod;
        self.damages.magic_mod *= enemy.magic_values.modifier * enemy.modifiers.magic_mod;
        self.damages.true_mod *= enemy.modifiers.true_mod;
        self.damages.global_mod *= enemy.modifiers.global_mod;
        self
    }
}

#[derive(Clone, Copy)]
struct DefensiveItemStats {
    values: [f32; 4],
    present: u8,
}

// Each item has at most one entry per relevant stat. Retain presence separately
// so an absent stat does not introduce a floating-point addition of zero.
const DEFENSIVE_ITEM_STATS: [DefensiveItemStats; ItemId::COUNT] = {
    let mut result = [DefensiveItemStats::NONE; ItemId::COUNT];
    let mut i = 0;

    while i < ItemId::COUNT {
        result[i] = DefensiveItemStats::new(ItemId::from_repr(i as _).unwrap().stats());
        i += 1;
    }

    result
};

impl DefensiveItemStats {
    const NONE: Self = Self {
        values: [0.0; 4],
        present: 0,
    };

    const fn new(stats: &[(StatName, u16)]) -> Self {
        let mut result = Self::NONE;
        let mut j = 0;

        while j < stats.len() {
            let (name, value) = stats[j];
            let slot = match name {
                StatName::Armor => 0,
                StatName::MagicResist => 1,
                StatName::Health => 2,
                StatName::Mana => 3,
                _ => 4,
            };

            if slot < 4 {
                assert!(result.present & (1 << slot) == 0);

                result.present |= 1 << slot;
                result.values[slot] = value as f32;
            }

            j += 1;
        }

        result
    }

    /// Stats of `item` on `game_map`, from the table unless its mode changes them
    #[inline(always)]
    const fn of(item: ItemId, game_map: GameMap) -> Self {
        match game_map.mode() {
            MapMode::Default => DEFENSIVE_ITEM_STATS[item as usize],
            _ => Self::of_mode(item, game_map),
        }
    }

    /// Stats of `item` on a map whose mode may change them, kept out of line
    /// so that [`Self::of`] inlines into the loop over items
    #[cold]
    #[inline(never)]
    const fn of_mode(item: ItemId, game_map: GameMap) -> Self {
        Self::new(item.stats_on(game_map))
    }
}

impl SimpleStats {
    pub const fn base_stats(champion_id: ChampionId, level: NonZeroU8, is_mega_gnar: bool) -> Self {
        champion_id.base_stats(level, is_mega_gnar).simple()
    }

    /// Mutates the variable `stats` and returns the bonus mana recovered from all items.
    /// In general, information about the enemy's mana is useless, but there are some items
    /// that increase their HP based on this stat. Because of that, having information about
    /// the bonus mana allows a better estimate about the enemy's current HP
    pub const fn infer_stats(
        &mut self,
        items: &[ItemId],
        earth_dragons: u16,
        game_map: GameMap,
    ) -> f32 {
        let mut bonus_mana = 0.0;
        let mut i = 0;

        while i < items.len() {
            let stats = DefensiveItemStats::of(items[i], game_map);

            if stats.present & 1 != 0 {
                self.armor += stats.values[0];
            }

            if stats.present & 2 != 0 {
                self.magic_resist += stats.values[1];
            }

            if stats.present & 4 != 0 {
                self.max_health += stats.values[2];
            }

            if stats.present & 8 != 0 {
                bonus_mana += stats.values[3];
            }

            i += 1;
        }

        let dragon_mod = RiotFormulas::get_earth_multiplier(earth_dragons);

        self.magic_resist *= dragon_mod;
        self.armor *= dragon_mod;

        bonus_mana
    }
}

impl WikiStats {
    /// Recover total bonus attack speed from attacks per second and the champion ratio.
    /// Observed attack speed cannot reveal bonuses hidden by the game's attack speed cap;
    /// callers with the true bonus can supply it directly through the context.
    pub const fn bonus_attack_speed_ratio(&self, attack_speed: f32) -> AttackSpeedRatio {
        if !attack_speed.is_finite()
            || !self.attack_speed.base.is_finite()
            || !self.attack_speed_ratio.is_finite()
            || self.attack_speed_ratio <= 0.0
        {
            return AttackSpeedRatio::new(0.0);
        }

        AttackSpeedRatio::new((attack_speed - self.attack_speed.base) / self.attack_speed_ratio)
    }

    pub const fn attack_speed(&self, bonus: f32, level: NonZeroU8) -> f32 {
        self.attack_speed.base
            + self.attack_speed_ratio
                * (Stat {
                    base: 0.0,
                    per_level: self.attack_speed.per_level,
                }
                .base_stat(level.get())
                    + bonus)
                / 100.0
    }

    pub const fn base_stats(&self, level: NonZeroU8) -> BasicStats {
        BasicStats {
            armor: self.armor.base_stat(level.get()),
            attack_damage: self.attack_damage.base_stat(level.get()),
            attack_speed: self.attack_speed(0.0, level),
            magic_resist: self.magic_resist.base_stat(level.get()),
            max_health: self.health.base_stat(level.get()),
            max_mana: self.mana.base_stat(level.get()),
        }
    }

    pub const fn mega_gnar_base_stats(&self, level: NonZeroU8) -> BasicStats {
        use champions::gnar::*;

        let mut gnar_stats = *self;

        gnar_stats.health.base += MEGA_HEALTH_BASE;
        gnar_stats.health.per_level += MEGA_HEALTH_PER_LEVEL;
        gnar_stats.armor.base += MEGA_ARMOR_BASE;
        gnar_stats.armor.per_level += MEGA_ARMOR_PER_LEVEL;
        gnar_stats.magic_resist.base += MEGA_MAGIC_RESIST_BASE;
        gnar_stats.magic_resist.per_level += MEGA_MAGIC_RESIST_PER_LEVEL;
        gnar_stats.attack_damage.base += MEGA_ATTACK_DAMAGE_BASE;
        gnar_stats.attack_damage.per_level += MEGA_ATTACK_DAMAGE_PER_LEVEL;
        gnar_stats.attack_speed.per_level -= MEGA_ATTACK_SPEED_LOSS_PER_LEVEL;
        gnar_stats.base_stats(level)
    }
}

impl ChampionId {
    pub const fn avg_stacks(&self, game_time: f32) -> f32 {
        // 30 minutes per game is the average
        const AVERAGE_GAME_TIME: u32 = 60 * 30;

        pub const fn time(game_time: f32, average: u32) -> f32 {
            average as f32 * game_time / AVERAGE_GAME_TIME as f32
        }

        // We don't know how many stacks the player has at that moment, so
        // the best we can do is to guess how much he would have at 30 minutes,
        // and calculate the average per minute so we can add. Obviously it will
        // grow linearly, while in game it is more common to get more stacks
        // towards late-game
        //
        // Note: These values might be outdated, and they come from my personal
        // experience playing the game, not a concrete set of data
        match self {
            ChampionId::Chogath => time(game_time, 12),
            ChampionId::Nasus => time(game_time, 600),
            ChampionId::Smolder => time(game_time, 120),
            ChampionId::AurelionSol => time(game_time, 190),
            ChampionId::Thresh => time(game_time, 150),
            ChampionId::Kindred => time(game_time, 5),
            _ => 1.0,
        }
    }

    /// Mega Gnar uses melee item and rune effects; other forms use cached data.
    pub const fn effective_attack_type(self, is_mega_gnar: bool) -> AttackType {
        if matches!(self, ChampionId::Gnar) && is_mega_gnar {
            core::hint::cold_path();
            AttackType::Melee
        } else {
            self.data().attack_type
        }
    }

    pub const fn base_stats(&self, level: NonZeroU8, is_mega_gnar: bool) -> BasicStats {
        Self::base_stats_of(self.stats(), *self, level, is_mega_gnar)
    }

    /// Base stats of the level on `game_map`, whose mode may change them
    #[inline(always)]
    pub const fn base_stats_on(
        &self,
        level: NonZeroU8,
        is_mega_gnar: bool,
        game_map: GameMap,
    ) -> BasicStats {
        // Every enemy reads its base stats, so Summoner's Rift keeps the same code
        // as when maps had no stats of their own
        if game_map.mode() as u8 == MapMode::Default as u8 {
            return self.base_stats(level, is_mega_gnar);
        }

        self.map_base_stats_cold(level, is_mega_gnar, game_map)
    }

    /// [`Self::base_stats_on`] on a map whose mode may change the stats
    #[cold]
    #[inline(never)]
    const fn map_base_stats_cold(
        &self,
        level: NonZeroU8,
        is_mega_gnar: bool,
        game_map: GameMap,
    ) -> BasicStats {
        let map_stats = self.map_stats(game_map);
        let source = match map_stats {
            Some(map_stats) => &map_stats.stats,
            None => self.stats(),
        };

        let mut stats = Self::base_stats_of(source, *self, level, is_mega_gnar);

        if let Some(map_stats) = map_stats {
            let mut index = 0;

            while index < map_stats.level_bonuses.len() {
                let bonus = &map_stats.level_bonuses[index];
                let value = bonus.value(level.get());

                match bonus.stat {
                    StatName::Health => stats.max_health += value,
                    StatName::Armor => stats.armor += value,
                    StatName::MagicResist => stats.magic_resist += value,
                    StatName::AttackDamage => stats.attack_damage += value,
                    _ => {}
                }

                index += 1;
            }
        }

        stats
    }

    const fn base_stats_of(
        stats: &WikiStats,
        champion_id: ChampionId,
        level: NonZeroU8,
        is_mega_gnar: bool,
    ) -> BasicStats {
        match champion_id {
            ChampionId::Gnar if is_mega_gnar => {
                core::hint::cold_path();
                stats.mega_gnar_base_stats(level)
            }
            _ => stats.base_stats(level),
        }
    }

    /// Base stats including the base health that `game_map` grants
    pub const fn map_base_stats(
        &self,
        level: NonZeroU8,
        is_mega_gnar: bool,
        game_map: GameMap,
    ) -> BasicStats {
        let mut stats = self.base_stats_on(level, is_mega_gnar, game_map);
        let attack_type = self.effective_attack_type(is_mega_gnar);

        stats.max_health += game_map.base_health(attack_type, level.get());
        stats
    }

    pub const fn map_damage_modifier(&self, enemy_id: &Self, game_map: GameMap) -> f32 {
        let WikiModifiers {
            ofa,
            usb,
            aram,
            ar,
            nb,
            swift,
            urf,
        } = self.data().modifiers;

        let WikiModifiers {
            ofa: e_ofa,
            usb: e_usb,
            aram: e_aram,
            ar: e_ar,
            nb: e_nb,
            swift: e_swift,
            urf: e_urf,
        } = enemy_id.data().modifiers;

        match game_map {
            GameMap::OneForAll => {
                core::hint::cold_path();
                ofa.damage_dealt * e_ofa.damage_taken
            }
            GameMap::UnsealedSpellbook => {
                core::hint::cold_path();
                usb.damage_dealt * e_usb.damage_taken
            }
            GameMap::Aram | GameMap::AramMayhem => aram.damage_dealt * e_aram.damage_taken,
            GameMap::Arena => ar.damage_dealt * e_ar.damage_taken,
            GameMap::NexusBlitz => {
                core::hint::cold_path();
                nb.damage_dealt * e_nb.damage_taken
            }
            GameMap::SwiftPlay => swift.damage_dealt * e_swift.damage_taken,
            GameMap::Urf => {
                core::hint::cold_path();
                urf.damage_dealt * e_urf.damage_taken
            }
            _ => 1.0,
        }
    }
}

impl BasicStats {
    /// Infers the champion's base stats at a given level. `is_mega_gnar` is valid only
    /// when `champion_id` is [`ChampionId::Gnar`], otherwise it has no effect.
    pub const fn base_stats(champion_id: ChampionId, level: NonZeroU8, is_mega_gnar: bool) -> Self {
        champion_id.base_stats(level, is_mega_gnar)
    }

    pub const fn simple(&self) -> SimpleStats {
        SimpleStats {
            armor: self.armor,
            max_health: self.max_health,
            magic_resist: self.magic_resist,
        }
    }

    pub const fn bonus_stats(&self, base_stats: Self) -> Self {
        Self {
            armor: self.armor - base_stats.armor,
            attack_damage: self.attack_damage - base_stats.attack_damage,
            attack_speed: self.attack_speed - base_stats.attack_speed,
            magic_resist: self.magic_resist - base_stats.magic_resist,
            max_health: self.max_health - base_stats.max_health,
            max_mana: self.max_mana - base_stats.max_mana,
        }
    }
}

impl EnemyStats {
    /// Ratio of the maximum health that is missing, see [`RiotFormulas::missing_health_ratio`]
    pub const fn missing_health_ratio(&self) -> f32 {
        RiotFormulas::missing_health_ratio(self.current_health, self.max_health)
    }

    pub const fn bonus_stats(&self, base_stats: SimpleStats) -> SimpleStats {
        SimpleStats {
            armor: self.armor - base_stats.armor,
            max_health: self.max_health - base_stats.max_health,
            magic_resist: self.magic_resist - base_stats.magic_resist,
        }
    }
}

impl PlayerStats {
    /// Ratio of the maximum health that is missing, see [`RiotFormulas::missing_health_ratio`]
    pub const fn missing_health_ratio(&self) -> f32 {
        RiotFormulas::missing_health_ratio(self.current_health, self.max_health)
    }

    /// Returns a new struct [`Stats`] with the same original values except the ones
    /// that involve percent penetration, which are resolved and converted to the
    /// `0..100` range used in this library
    pub const fn base100(&self) -> Self {
        Self {
            armor_penetration_percent: (1.0 - self.armor_penetration_percent).clamp(0.0, 1.0)
                * 100.0,
            magic_penetration_percent: (1.0 - self.magic_penetration_percent).clamp(0.0, 1.0)
                * 100.0,
            ..*self
        }
    }

    /// Receives the current player stats and the qualified dragons and returns a large array
    /// of stats as if the player owned the qualified item, defined in the constant
    /// [`SIMULATED_ITEMS_ENUM`]. The qualified items are defined by their tier, gold, and if
    /// they're purchasable in the standard gamemode [`GameMap::SummonersRift`].
    pub const fn get_simulated_stats(
        &self,
        dragons: Dragons,
        game_map: GameMap,
    ) -> [Self; ItemId::L_SIML] {
        let fire = RiotFormulas::get_fire_multiplier(dragons.ally_fire_dragons);
        let earth = RiotFormulas::get_earth_multiplier(dragons.ally_earth_dragons);

        let mut result = [*self; ItemId::L_SIML];
        let mut i = 0;

        while i < ItemId::L_SIML {
            result[i] =
                self.with_item_stats_with_multipliers(ItemId::SIML[i], fire, earth, game_map);
            i += 1;
        }

        result
    }

    /// Add raw item stats to live totals, applying dragons only to new stats.
    pub const fn with_item_stats(&self, item: ItemId, dragons: Dragons, game_map: GameMap) -> Self {
        self.with_item_stats_with_multipliers(
            item,
            RiotFormulas::get_fire_multiplier(dragons.ally_fire_dragons),
            RiotFormulas::get_earth_multiplier(dragons.ally_earth_dragons),
            game_map,
        )
    }

    /// Apply item deltas using dragon multipliers shared by a simulation batch.
    pub(crate) const fn with_item_stats_with_multipliers(
        &self,
        item: ItemId,
        fire: f32,
        earth: f32,
        game_map: GameMap,
    ) -> Self {
        let mut added = PlayerStats::default();

        added.get_item_bonus_stats(&[item], &mut Modifiers::default(), &mut 0.0, game_map);

        Self {
            ability_power: self.ability_power + added.ability_power * fire,
            attack_damage: self.attack_damage + added.attack_damage * fire,
            armor: self.armor + added.armor * earth,
            magic_resist: self.magic_resist + added.magic_resist * earth,
            max_health: self.max_health + added.max_health,
            max_mana: self.max_mana + added.max_mana,
            attack_speed: self.attack_speed + added.attack_speed,
            crit_chance: (self.crit_chance + added.crit_chance).clamp(0.0, 100.0),
            crit_damage: self.crit_damage + added.crit_damage,
            armor_penetration_flat: self.armor_penetration_flat + added.armor_penetration_flat,
            magic_penetration_flat: self.magic_penetration_flat + added.magic_penetration_flat,
            armor_penetration_percent: RiotFormulas::combine_percentage(
                self.armor_penetration_percent,
                added.armor_penetration_percent,
            ),
            magic_penetration_percent: RiotFormulas::combine_percentage(
                self.magic_penetration_percent,
                added.magic_penetration_percent,
            ),
            ..*self
        }
    }

    pub const fn bonus_stats(&self, base_stats: BasicStats) -> BasicStats {
        BasicStats {
            armor: self.armor - base_stats.armor,
            attack_damage: self.attack_damage - base_stats.attack_damage,
            attack_speed: self.attack_speed - base_stats.attack_speed,
            magic_resist: self.magic_resist - base_stats.magic_resist,
            max_health: self.max_health - base_stats.max_health,
            max_mana: self.max_mana - base_stats.max_mana,
        }
    }
}

impl<T> DamageKind<T> {
    /// Receives a bitset of damaging items or runes and a function that returns
    /// all metadata and closures for each element
    pub fn new<const N: usize>(
        bitset: &BitSetArray<N>,
        attack_type: AttackType,
        f: impl Fn(u32, AttackType) -> (TypeMetadata<T>, [Closure; 2]),
    ) -> Self {
        let count = bitset.count_const() as usize;
        let mut metadata = Box::new_uninit_slice(count);
        let mut closures = Box::new_uninit_slice(count << 1);

        unsafe {
            for (i, j) in bitset.iter_const().enumerate() {
                let (meta, slice) = f(j, attack_type);
                let base = i << 1;

                closures.get_unchecked_mut(base).write(slice[0]);
                closures.get_unchecked_mut(base + 1).write(slice[1]);
                metadata.get_unchecked_mut(i).write(meta);
            }

            Self {
                metadata: metadata.assume_init(),
                closures: closures.assume_init(),
            }
        }
    }
}

impl DamageKind<ItemId> {
    pub fn items(bitset: &BitSet, attack_type: AttackType) -> Self {
        Self::new(bitset, attack_type, |i, attack_type| {
            let item = ItemId::from_repr(i as _).unwrap().data();
            let slice = match attack_type {
                AttackType::Ranged => item.ranged,
                AttackType::Melee => item.melee,
            };

            (item.metadata, slice)
        })
    }
}

impl DamageKind<RuneId> {
    pub fn runes(bitset: &BitSet, attack_type: AttackType) -> Self {
        Self::new(bitset, attack_type, |i, attack_type| {
            let rune = RuneId::from_repr(i as _).unwrap().data();
            let slice = match attack_type {
                AttackType::Ranged => rune.ranged,
                AttackType::Melee => rune.melee,
            };

            (rune.metadata, slice)
        })
    }
}

pub const fn get_damaging_runes(input: &[RuneId]) -> BitSet {
    let mut out = BitSet::EMPTY;
    let mut i = 0;

    while i < input.len() {
        let rune = input[i] as _;

        if RuneId::DAMAGING.contains_const(rune) {
            out.insert_const(rune);
        }

        i += 1;
    }
    out
}

/// Converts a slice of [`ItemId`] into a [`ItemsBitSet`], removing the ones that
/// do not deal any damage
pub const fn get_damaging_items(input: &[ItemId]) -> BitSet {
    let mut out = BitSet::EMPTY;
    let mut i = 0;

    while i < input.len() {
        let item = input[i] as _;

        if ItemId::DAMAGING.contains_const(item) {
            out.insert_const(item);
        }

        i += 1;
    }

    out
}

/// Takes information about the current enemy and returns a struct that represents
/// all the useful information we should infer from the current enemy state. This
/// will be used to create a struct [`Ctx`] that will be used to evaluate
/// the closures of the current champion. Champion and item specific bonus stats
/// are applied in this function
impl EnemyState<'_> {
    pub const fn full_state(&self, shred: ResistShred, accept_negatives: bool) -> EnemyFullState {
        let EnemyState {
            current_stats,
            base_stats,
            items,
            stacks,
            champion_id,
            earth_dragons,
            level,
            item_exceptions,
            game_map,
        } = *self;

        // Health granted by the map is base health, so it is part of the base
        // stats whether the totals are supplied or inferred
        let attack_type = champion_id.data().attack_type;
        let mut base_stats = base_stats;
        let mut e_modifiers = const { DamageModifiers::default() };

        base_stats.max_health += game_map.base_health(attack_type, level.get());

        let ResistShred {
            armor_penetration_flat,
            armor_penetration_percent,
            magic_penetration_flat,
            magic_penetration_percent,
        } = shred;

        if matches!(champion_id, ChampionId::Kassadin) {
            e_modifiers.magic_mod *= 1.0 - champions::kassadin::VOID_STONE_MAGIC_REDUCTION;
        }

        // Supplied totals already include stat passives and dragons. Keep inference
        // confined to None; defensive modifiers and item flags apply to both paths.
        let e_current_stats = match current_stats {
            Some(s) => s,
            None => {
                let mut e_default_stats = base_stats;
                let (mode_armor, mode_magic_resist) =
                    game_map.bonus_resists(attack_type, level.get());

                e_default_stats.armor += mode_armor;
                e_default_stats.magic_resist += mode_magic_resist;

                let bonus_mana = e_default_stats.infer_stats(items, earth_dragons, game_map);
                // Warmog's Vitality only scales the health granted by items, which
                // also includes the health that Awe converts from mana below
                let mut item_health = e_default_stats.max_health - base_stats.max_health;
                let mut has_warmogs = false;
                let mut i = 0;

                while i < item_exceptions.len() {
                    if let Some(item_id) = item_exceptions[i].get_item_id() {
                        match item_id {
                            ItemId::WintersApproach => {
                                core::hint::cold_path();

                                let awe =
                                    items::winters_approach::awe_bonus_mana_ratio(game_map.mode())
                                        * bonus_mana;

                                e_default_stats.max_health += awe;
                                item_health += awe;
                            }
                            ItemId::Fimbulwinter => {
                                core::hint::cold_path();

                                let awe =
                                    items::fimbulwinter::awe_bonus_mana_ratio(game_map.mode())
                                        * bonus_mana;

                                e_default_stats.max_health += awe;
                                item_health += awe;
                            }
                            ItemId::WarmogsArmor => {
                                core::hint::cold_path();

                                has_warmogs = true
                            }
                            _ => {}
                        }
                    }

                    i += 1;
                }

                // Warmog's waits for every conversion of Awe, in any order
                if has_warmogs {
                    core::hint::cold_path();

                    e_default_stats.max_health +=
                        items::warmogs_armor::WARMOGS_VITALITY_BONUS_HEALTH_RATIO * item_health;
                }

                // Dragonheart and Demon King's Crown increase the total stats,
                // so they apply after Warmog's
                let total_stats_modifier = ValueException::total_stats_modifier(item_exceptions);

                e_default_stats.max_health *= total_stats_modifier;
                e_default_stats.armor *= total_stats_modifier;
                e_default_stats.magic_resist *= total_stats_modifier;

                match champion_id {
                    ChampionId::Swain => {
                        e_default_stats.max_health +=
                            champions::swain::SOUL_FRAGMENT_HEALTH * stacks as f32;
                    }
                    ChampionId::Chogath => {
                        // Feast grants bonus health per stack based on its rank
                        let hp_per_stack = match level.get() {
                            ..6 => 0.0,
                            6..11 => champions::chogath::FEAST_HEALTH_PER_STACK[0],
                            11..16 => champions::chogath::FEAST_HEALTH_PER_STACK[1],
                            16.. => champions::chogath::FEAST_HEALTH_PER_STACK[2],
                        };

                        e_default_stats.max_health += stacks as f32 * hp_per_stack;
                    }
                    ChampionId::Sion => {
                        e_default_stats.max_health += stacks as f32;
                    }
                    ChampionId::Ornn => {
                        use champions::ornn::{
                            MASTER_CRAFTSMAN_BASE_BONUS, MASTER_CRAFTSMAN_MAX_BONUS,
                            MASTER_CRAFTSMAN_UPGRADE_BONUS,
                        };

                        // Starts game with +10% bonus armor/mr/hp already
                        // After level 13, player will start upgrading items
                        // At level 18, the maximum bonus must have been reached
                        // For every upgrade, a +4% bonus is applied.
                        // #![manual_impl]
                        let upgrades = level.get().saturating_sub(12) as f32;
                        let ornn_resist_multiplier = 1.0
                            + (MASTER_CRAFTSMAN_BASE_BONUS
                                + upgrades * MASTER_CRAFTSMAN_UPGRADE_BONUS)
                                .min(MASTER_CRAFTSMAN_MAX_BONUS);

                        // Master Craftsman only scales bonus stats
                        e_default_stats.armor = base_stats.armor
                            + (e_default_stats.armor - base_stats.armor) * ornn_resist_multiplier;

                        e_default_stats.magic_resist = base_stats.magic_resist
                            + (e_default_stats.magic_resist - base_stats.magic_resist)
                                * ornn_resist_multiplier;

                        e_default_stats.max_health = base_stats.max_health
                            + (e_default_stats.max_health - base_stats.max_health)
                                * ornn_resist_multiplier;
                    }
                    ChampionId::Poppy => {
                        // Inferred enemies are at full health, so the bonus is
                        // never doubled by her being at low health
                        const MULTIPLIER: f32 = 1.0 + champions::poppy::STUBBORN_TO_A_FAULT_RESISTS;

                        e_default_stats.armor *= MULTIPLIER;
                        e_default_stats.magic_resist *= MULTIPLIER;
                    }
                    ChampionId::Malphite => {
                        // W rank by level, assuming it is maxed last
                        // #![manual_impl]
                        let w_rank = match level.get() {
                            ..3 => 0,
                            3..14 => 1,
                            14 => 2,
                            15..17 => 3,
                            17 => 4,
                            18.. => 5,
                        };

                        let malphite_resist_multiplier = match w_rank {
                            0 => 1.0,
                            _ => 1.0 + champions::malphite::THUNDERCLAP_BONUS_ARMOR[w_rank - 1],
                        };

                        e_default_stats.armor *= malphite_resist_multiplier;
                    }
                    _ => {}
                }

                EnemyStats {
                    armor: e_default_stats.armor,
                    current_health: e_default_stats.max_health,
                    magic_resist: e_default_stats.magic_resist,
                    max_health: e_default_stats.max_health,
                }
            }
        };

        let armor_values = RiotFormulas::real_resist(
            armor_penetration_percent,
            armor_penetration_flat,
            e_current_stats.armor,
            accept_negatives,
        );

        let magic_values = RiotFormulas::real_resist(
            magic_penetration_percent,
            magic_penetration_flat,
            e_current_stats.magic_resist,
            accept_negatives,
        );

        let e_bonus_stats = e_current_stats.bonus_stats(base_stats);
        let mut origin = BitSet::EMPTY;
        let mut i = 0;

        while i < items.len() {
            origin.insert_const(items[i] as _);
            i += 1;
        }

        EnemyFullState {
            current_stats: e_current_stats,
            bonus_stats: e_bonus_stats,
            modifiers: e_modifiers,
            armor_values,
            magic_values,
            steelcaps: has_item(&origin, [ItemId::PlatedSteelcaps, ItemId::ArmoredAdvance]),
            rocksolid: has_item(&origin, [ItemId::WardensMail]),
            randuin: has_item(&origin, [ItemId::RanduinsOmen]),
            accept_negatives,
        }
    }
}

impl SelfState {
    /// Construct a new [`Ctx`] type that can be used to evaluate any champion's
    /// closures and get their intermediary damage values, before applying the reductions
    /// from armor and magic resist. See [`Closure`] for more details about those
    /// functions
    pub const fn ctx(&self, e_state: &EnemyFullState, game_map: GameMap) -> Ctx {
        Ctx::new(&self.ctx_attacker(game_map), &e_state.ctx_target())
    }

    /// Part of [`Ctx`] that comes from this state, the same against every target.
    /// `game_map` selects the closures that a game mode changes
    pub const fn ctx_attacker(&self, game_map: GameMap) -> CtxAttacker {
        let SelfState {
            stacks,
            bonus_attack_speed_ratio,
            ability_levels,
            current_stats:
                PlayerStats {
                    ability_power,
                    armor,
                    armor_penetration_flat,
                    armor_penetration_percent,
                    attack_damage,
                    attack_speed,
                    crit_chance,
                    crit_damage,
                    current_health,
                    magic_penetration_flat,
                    magic_penetration_percent,
                    magic_resist,
                    max_health,
                    max_mana,
                    current_mana,
                },
            base_stats:
                BasicStats {
                    armor: base_armor,
                    max_health: base_health,
                    attack_damage: base_ad,
                    magic_resist: base_magic_resist,
                    max_mana: base_mana,
                    attack_speed: base_attack_speed,
                },
            level,
            adaptive_type,
            ..
        } = *self;

        CtxAttacker {
            bonus_attack_speed_ratio,
            ability_levels,
            level: level.get() as _,
            base_health,
            base_ad,
            base_armor,
            base_attack_speed,
            base_magic_resist,
            base_mana,
            armor_penetration_flat,
            armor_penetration_percent,
            magic_penetration_flat,
            magic_penetration_percent,
            max_mana,
            current_mana,
            max_health,
            current_health,
            armor,
            magic_resist,
            crit_chance,
            crit_damage,
            attack_speed,
            ability_power,
            attack_damage,
            stacks,
            flags: CtxFlags::attacker(adaptive_type),
            map: game_map.mode(),
        }
    }

    /// Estimate a purchase from observed live stats rather than replacing them
    /// with inferred totals. Unknown stacks and temporary effects remain observed.
    pub const fn simulated_purchase(
        &self,
        item: ItemId,
        owned: &[ItemId],
        champion: ChampionId,
        dragons: Dragons,
        game_map: GameMap,
    ) -> Self {
        self.simulated_purchase_with_multipliers(
            item,
            owned,
            champion,
            RiotFormulas::get_fire_multiplier(dragons.ally_fire_dragons),
            RiotFormulas::get_earth_multiplier(dragons.ally_earth_dragons),
            game_map,
        )
    }

    /// Prepare a purchase using dragon multipliers shared by all candidates.
    pub(crate) const fn simulated_purchase_with_multipliers(
        &self,
        item: ItemId,
        owned: &[ItemId],
        champion: ChampionId,
        fire: f32,
        earth: f32,
        game_map: GameMap,
    ) -> Self {
        let current = self.current_stats;
        let mut stats = current.with_item_stats_with_multipliers(item, fire, earth, game_map);
        let data = champion.data();

        // Item attack speed is a percent bonus, while live attack_speed is attacks/sec.
        let total_attack_speed = match champion.map_stats(game_map) {
            Some(map_stats) => map_stats.total_attack_speed,
            None => 1.0,
        };

        stats.attack_speed = current.attack_speed
            + champion.stats_on(game_map).attack_speed_ratio
                * (stats.attack_speed - current.attack_speed)
                / 100.0
                * total_attack_speed;

        let mut already_owned = false;
        let mut i = 0;

        while i < owned.len() {
            if owned[i] as u16 == item as u16 {
                already_owned = true;
                break;
            }

            i += 1;
        }

        let added_mana = stats.max_mana - current.max_mana;

        // Existing conversion passives also benefit from the candidate's stats.
        i = 0;

        while i < owned.len() {
            let existing = owned[i];

            match existing {
                ItemId::WintersApproach => {
                    core::hint::cold_path();

                    stats.max_health +=
                        items::winters_approach::awe_bonus_mana_ratio(game_map.mode()) * added_mana
                }
                ItemId::Fimbulwinter => {
                    core::hint::cold_path();

                    stats.max_health +=
                        items::fimbulwinter::awe_bonus_mana_ratio(game_map.mode()) * added_mana
                }
                _ => {}
            }

            i += 1;
        }

        if !already_owned {
            match item {
                ItemId::WintersApproach => {
                    core::hint::cold_path();

                    stats.max_health +=
                        items::winters_approach::awe_bonus_mana_ratio(game_map.mode())
                            * (stats.max_mana - self.base_stats.max_mana)
                }
                ItemId::Fimbulwinter => {
                    core::hint::cold_path();

                    stats.max_health += items::fimbulwinter::awe_bonus_mana_ratio(game_map.mode())
                        * (stats.max_mana - self.base_stats.max_mana)
                }
                _ => {}
            }
        }

        // Amplify the entire added health, including mana converted above.
        i = 0;

        while i < owned.len() {
            let existing = owned[i];

            if matches!(existing, ItemId::WarmogsArmor) {
                core::hint::cold_path();

                stats.max_health += items::warmogs_armor::WARMOGS_VITALITY_BONUS_HEALTH_RATIO
                    * (stats.max_health - current.max_health);
            }

            i += 1;
        }

        // Observed health doesn't tell which part came from items, so the whole
        // bonus health is amplified
        if !already_owned && matches!(item, ItemId::WarmogsArmor) {
            core::hint::cold_path();

            stats.max_health += items::warmogs_armor::WARMOGS_VITALITY_BONUS_HEALTH_RATIO
                * (stats.max_health - self.base_stats.max_health);
        }

        let health_delta = stats.max_health - current.max_health;
        let mut added_ap = 0.0;
        let mut added_ad = 0.0;
        let mut owns_bloodmail = false;

        i = 0;

        while i < owned.len() {
            let existing = owned[i];

            match existing {
                ItemId::Riftmaker => {
                    added_ap += items::riftmaker::VOID_INFUSION_BONUS_HEALTH_RATIO * health_delta
                }
                ItemId::ArchangelsStaff => {
                    added_ap +=
                        items::archangels_staff::awe_bonus_mana_ratio(game_map.mode()) * added_mana
                }
                ItemId::SeraphsEmbrace => {
                    added_ap +=
                        items::seraphs_embrace::awe_bonus_mana_ratio(game_map.mode()) * added_mana
                }
                ItemId::Manamune => added_ad += items::manamune::AWE_MAX_MANA_RATIO * added_mana,
                ItemId::Muramana => added_ad += items::muramana::AWE_MAX_MANA_RATIO * added_mana,
                ItemId::OverlordsBloodmail => {
                    core::hint::cold_path();

                    owns_bloodmail = true;
                    added_ad +=
                        items::overlords_bloodmail::tyranny_bonus_health_ratio(game_map.mode())
                            * health_delta
                }
                _ => {}
            }

            i += 1;
        }

        if !already_owned {
            let bonus_health = stats.max_health - self.base_stats.max_health;
            let bonus_mana = stats.max_mana - self.base_stats.max_mana;

            match item {
                ItemId::Riftmaker => {
                    added_ap += items::riftmaker::VOID_INFUSION_BONUS_HEALTH_RATIO * bonus_health
                }
                // Purchases do not change the flat missing health, so an owned
                // Demonic Embrace keeps the ability power already observed
                ItemId::DemonicEmbrace => {
                    core::hint::cold_path();

                    added_ap +=
                        RiotFormulas::get_sinister_pact(current.max_health - current.current_health)
                }
                ItemId::ArchangelsStaff => {
                    added_ap +=
                        items::archangels_staff::awe_bonus_mana_ratio(game_map.mode()) * bonus_mana
                }
                ItemId::SeraphsEmbrace => {
                    added_ap +=
                        items::seraphs_embrace::awe_bonus_mana_ratio(game_map.mode()) * bonus_mana
                }
                ItemId::Manamune => {
                    added_ad += items::manamune::AWE_MAX_MANA_RATIO * stats.max_mana
                }
                ItemId::Muramana => {
                    added_ad += items::muramana::AWE_MAX_MANA_RATIO * stats.max_mana
                }
                ItemId::OverlordsBloodmail => {
                    core::hint::cold_path();

                    added_ad +=
                        items::overlords_bloodmail::tyranny_bonus_health_ratio(game_map.mode())
                            * bonus_health
                }
                _ => {}
            }
        }

        stats.ability_power += added_ap * fire;
        stats.attack_damage += added_ad * fire;

        // Retribution grants a ratio of the attack damage from other sources.
        // Observed attack damage already includes it when Bloodmail is owned,
        // so only the attack damage added by the purchase is amplified
        if owns_bloodmail || matches!(item, ItemId::OverlordsBloodmail) {
            core::hint::cold_path();

            let retribution = 1.0
                + RiotFormulas::get_bloodmail_retribution(current.missing_health_ratio(), game_map);

            stats.attack_damage = match owns_bloodmail {
                true => {
                    current.attack_damage
                        + (stats.attack_damage - current.attack_damage) * retribution
                }
                false => stats.attack_damage * retribution,
            };
        }

        // The observed AP already includes owned amplifiers; only amplify the delta.
        const RABADONS_AMPLIFIER: f32 = 1.0 + items::rabadons_deathcap::MAGICAL_OPUS_AP_AMPLIFIER;
        const WOOGLETS_AMPLIFIER: f32 = 1.0 + items::wooglets_witchcap::MAGICAL_OPUS_AP_AMPLIFIER;

        let ap_delta = stats.ability_power - current.ability_power;
        let mut ap_multiplier = 1.0;

        i = 0;

        while i < owned.len() {
            let existing = owned[i];

            match existing {
                ItemId::RabadonsDeathcap => ap_multiplier *= RABADONS_AMPLIFIER,
                ItemId::WoogletsWitchcap => {
                    core::hint::cold_path();
                    ap_multiplier *= WOOGLETS_AMPLIFIER
                }
                _ => {}
            }

            i += 1;
        }

        stats.ability_power = current.ability_power + ap_delta * ap_multiplier;

        if !already_owned {
            match item {
                ItemId::RabadonsDeathcap => {
                    core::hint::cold_path();
                    stats.ability_power *= RABADONS_AMPLIFIER
                }
                ItemId::WoogletsWitchcap => {
                    core::hint::cold_path();
                    stats.ability_power *= WOOGLETS_AMPLIFIER
                }
                _ => {}
            }
        }

        let bonus_stats = stats.bonus_stats(self.base_stats);
        let adaptive_type =
            match AdaptiveType::try_infer(bonus_stats.attack_damage, stats.ability_power) {
                Some(value) => value,
                None => data.adaptive_type,
            };

        Self {
            bonus_attack_speed_ratio: champion
                .stats_on(game_map)
                .bonus_attack_speed_ratio(stats.attack_speed),
            current_stats: stats,
            adaptive_type,
            ..*self
        }
    }
}

impl EnemyFullState {
    pub const fn with_shred(mut self, shred: ResistShred, accept_negatives: bool) -> Self {
        self.armor_values = RiotFormulas::real_resist(
            shred.armor_penetration_percent,
            shred.armor_penetration_flat,
            self.current_stats.armor,
            accept_negatives,
        );

        self.magic_values = RiotFormulas::real_resist(
            shred.magic_penetration_percent,
            shred.magic_penetration_flat,
            self.current_stats.magic_resist,
            accept_negatives,
        );

        self.accept_negatives = accept_negatives;
        self
    }

    pub const fn ctx(&self, self_state: &SelfState, game_map: GameMap) -> Ctx {
        self_state.ctx(self, game_map)
    }

    /// Multiplier of the attacker's runes that depend on the health of this enemy
    /// champion: [`RuneId::CoupDeGrace`] below its health threshold and
    /// [`RuneId::CutDown`] above it. Monsters are not affected by these runes
    pub const fn rune_damage_modifier(&self, owned_runes: &[RuneId]) -> f32 {
        use crate::libgen::runes::{coup_de_grace, cut_down};

        // Computed directly so that exact thresholds such as 60% are not lost to rounding
        let EnemyStats {
            current_health,
            max_health,
            ..
        } = self.current_stats;

        let health_ratio = (current_health / max_health.max(1.0)).clamp(0.0, 1.0);
        let mut modifier = 1.0;
        let mut i = 0;

        while i < owned_runes.len() {
            match owned_runes[i] {
                RuneId::CoupDeGrace if health_ratio < coup_de_grace::HEALTH_THRESHOLD => {
                    modifier *= RiotFormulas::COUP_DE_GRACE_BONUS_DAMAGE
                }
                RuneId::CutDown if health_ratio > cut_down::HEALTH_THRESHOLD => {
                    modifier *= RiotFormulas::CUT_DOWN_BONUS_DAMAGE
                }
                _ => {}
            }

            i += 1;
        }

        modifier
    }
}

impl EnemyFullState {
    /// Part of [`Ctx`] that comes from this target, the same for every attacker state
    pub const fn ctx_target(&self) -> CtxTarget {
        let EnemyFullState {
            current_stats:
                EnemyStats {
                    armor: enemy_armor,
                    current_health: enemy_current_health,
                    magic_resist: enemy_magic_resist,
                    max_health: enemy_max_health,
                },
            bonus_stats:
                SimpleStats {
                    max_health: enemy_bonus_health,
                    armor: enemy_bonus_armor,
                    magic_resist: enemy_bonus_magic_resist,
                },
            steelcaps,
            rocksolid,
            randuin,
            accept_negatives,
            ..
        } = *self;

        CtxTarget {
            enemy_bonus_health,
            enemy_bonus_armor,
            enemy_bonus_magic_resist,
            enemy_armor,
            enemy_current_health,
            enemy_max_health,
            enemy_magic_resist,
            flags: CtxFlags::target(steelcaps, randuin, rocksolid, accept_negatives),
        }
    }
}

impl Modifiers {
    /// Inserts ability modifier buffs or debuffs for each individual ability, based on their
    /// letter discriminant: `P`, `Q`, `W`, `E`, `R`.
    pub const fn ability_id_mod(&self, ability_id: AbilityId, damage_type: DamageType) -> f32 {
        let Modifiers { damages, abilities } = self;

        damages.modifier(damage_type) * abilities.modifier(ability_id)
    }
}

impl Attacks {
    /// Evaluates the damage of basic attacks, onhit damages and critical strikes
    pub const fn new(ctx: &Ctx, mut onhit_damage: RangeDamage, physical_mod: f32) -> Self {
        let basic_attack = ctx.attack_damage * physical_mod;
        let critical_strike = (basic_attack * ctx.crit_damage / 100.0) as i32;
        let basic_attack = basic_attack as i32;

        onhit_damage.minimum_damage += basic_attack;
        onhit_damage.maximum_damage += critical_strike;

        Self {
            basic_attack,
            critical_strike,
            onhit_damage,
        }
    }
}

/// Confirms that every single metadata array have the same length as the closures array,
/// for every champion. Also, for every item, the melee and range closures should also have
/// the same number of elements inside. If this is not done, some functions in this module
/// will panic or cause undefined behavior
const _: () = {
    let mut i = 0;

    while i < ChampionId::COUNT {
        let champion_id = ChampionId::from_repr(i as _).unwrap();

        let Champion {
            metadata, closures, ..
        } = champion_id.data();

        assert!(metadata.len() == closures.len());
        i += 1;
    }
};

impl DamageTable {
    /// Evaluates one row. The caller may add the candidate pair afterwards via the returned view.
    pub fn fill(
        &mut self,
        r: usize,
        ctx: Ctx,
        data: &DamageEvalData,
        modifiers: Modifiers,
    ) -> DamagesMut<'_> {
        self.fill_with_batch(r, ctx, data, modifiers, None)
    }

    /// Batch specialization: one indirect call for the whole champion.
    pub fn fill_batch(
        &mut self,
        r: usize,
        ctx: Ctx,
        data: &DamageEvalData,
        modifiers: Modifiers,
        batch: AbilityBatch,
    ) -> DamagesMut<'_> {
        self.fill_with_batch(r, ctx, data, modifiers, Some(batch))
    }

    fn fill_with_batch(
        &mut self,
        r: usize,
        ctx: Ctx,
        data: &DamageEvalData,
        modifiers: Modifiers,
        batch: Option<AbilityBatch>,
    ) -> DamagesMut<'_> {
        self.set_ctx(r, &ctx);

        // excludes the reserved candidate pair
        let n_items = data.items.closures.len();
        let row = self.row_mut(r);
        let mut onhit = const { RangeDamage::default() };

        if let Some(batch) = batch {
            batch(&ctx, modifiers, &mut onhit, row.abilities);
        } else {
            Self::eval_abilities(
                &ctx,
                &mut onhit,
                modifiers,
                data.abilities.metadata,
                data.abilities.closures,
                row.abilities,
            );
        }

        Self::eval(
            &ctx,
            &mut onhit,
            modifiers,
            &data.items.metadata,
            &data.items.closures,
            &mut row.items[..n_items],
        );

        Self::eval(
            &ctx,
            &mut onhit,
            modifiers,
            &data.runes.metadata,
            &data.runes.closures,
            row.runes,
        );

        let a = Attacks::new(&ctx, onhit, modifiers.damages.physical_mod);

        *row.attacks = [
            a.basic_attack,
            a.critical_strike,
            a.onhit_damage.minimum_damage,
            a.onhit_damage.maximum_damage,
        ];

        row
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fill_monster_damages(
        &mut self,
        self_state: &SelfState,
        eval_data: &DamageEvalData,
        shred: ResistShred,
        modifiers: Modifiers,
        batch: AbilityBatch,
        game_map: GameMap,
        monster_level: u8,
    ) {
        for (i, &monster) in Monster::VARIANTS.iter().enumerate() {
            let full_state = EnemyState {
                base_stats: monster.stats(monster_level),
                ..Default::default()
            }
            .full_state(shred, true);

            let ctx = self_state.ctx(&full_state, game_map);
            let modifiers = modifiers.against(&full_state, self_state.adaptive_type);

            self.fill_batch(i, ctx, eval_data, modifiers, batch);
        }
    }

    /// Evaluates the damages of all requested items or runes. This function causes
    /// `Undefined Behavior` if the length of `closures` is not twice as much as `metadata`
    /// or panic in debug mode. See similar function [`ability_id_eval_damage`]
    pub fn eval<T>(
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        modifiers: Modifiers,
        metadata: &[TypeMetadata<T>],
        closures: &[Closure],
        result: &mut [i32],
    ) {
        let out_len = closures.len();

        debug_assert_eq!(out_len, metadata.len() << 1);

        let mut meta_index = 0usize;
        let mut out_index = 0usize;

        while meta_index < metadata.len() {
            let TypeMetadata {
                damage_type,
                attributes,
                ..
            } = *unsafe { metadata.get_unchecked(meta_index) };

            let modifier = modifiers.damages.modifier(damage_type);
            let mut j = 0;

            while j < 2 {
                let closure = unsafe { closures.get_unchecked((meta_index << 1) + j) };
                let damage = (modifier * closure(ctx)) as i32;

                onhit.inc_attr(attributes, damage);
                result[out_index] = damage;
                out_index += 1;
                j += 1;
            }

            meta_index += 1;
        }

        debug_assert_eq!(out_index, out_len);
    }

    /// Evaluates the damage of all provided metadata of [`AbilityId`]. This function
    /// already multiplies the final damage result by the appropriate armor, or magic
    /// resist multiplier of the enemy, and considers global and local damage modifiers
    /// This function will cause `Undefined Behavior` if the length of `closures` and
    /// `metadata` are not equal.
    pub fn eval_abilities(
        ctx: &Ctx,
        onhit: &mut RangeDamage,
        modifiers: Modifiers,
        metadata: &[TypeMetadata<AbilityId>],
        closures: &[Closure],
        out: &mut [i32],
    ) {
        let len = metadata.len();
        let mut i = 0;

        debug_assert_eq!(len, closures.len());
        debug_assert_eq!(len, out.len());

        while i < len {
            let TypeMetadata {
                kind,
                damage_type,
                attributes,
            } = metadata[i];

            let closure = unsafe { closures.get_unchecked(i) };
            let modifier = modifiers.ability_id_mod(kind, damage_type);
            let damage = (modifier * closure(ctx)) as i32;

            onhit.inc_attr(attributes, damage);
            out[i] = damage;

            i += 1;
        }
    }
}

/// Returns an array with the damage against turrets of different plates.
/// The array has 6 ([`L_TWRD`]) elements because a tower can have zero,
/// or up to 5 plates
pub const fn get_tower_damages(
    adaptive_type: AdaptiveType,
    base_attack_damage: f32,
    bonus_attack_damage: f32,
    ability_power: f32,
    shred: ResistShred,
) -> [i32; L_TWRD] {
    let mut tower_damages = MaybeUninit::<[i32; L_TWRD]>::uninit();
    let tower_ptr = tower_damages.as_mut_ptr();
    let mut i = 0;

    let (pen_percent, pen_flat) = match adaptive_type {
        AdaptiveType::Physical => (
            shred.armor_penetration_percent,
            shred.armor_penetration_flat,
        ),
        AdaptiveType::Magic => (
            shred.magic_penetration_percent,
            shred.magic_penetration_flat,
        ),
    };

    while i < L_TWRD {
        let damage = RiotFormulas::tower_damage(
            i as _,
            base_attack_damage,
            bonus_attack_damage,
            ability_power,
            pen_percent,
            pen_flat,
        );

        unsafe {
            (&raw mut (*tower_ptr)[i]).write(damage);
        }

        i += 1;
    }

    unsafe { tower_damages.assume_init() }
}
