//! This module has functions related to the `calculator` section,
//! which receives custom game information to evaluate damages.
//! The functions exported in this module are more optimized than
//! the ones in the [`crate::realtime`] module, and have more
//! information, which means their results are more accurate.
//!
//! Note that this module have no dependencies on Riot's API,
//! so all the function inputs have to be obtained through your
//! own mechanism

use {
    crate::{ChampionId, ItemId, RuneId, helpers::*, model::*},
    alloc::boxed::Box,
    tutorlolv2_types::{AdaptiveType, AttackType, StatName},
};

pub const fn get_item_bonus_stats(
    stats: &mut Stats,
    items: &[ItemId],
    modifiers: &mut Modifiers,
    adaptive_force: &mut f32,
) {
    stats.get_item_bonus_stats(items, modifiers, adaptive_force);
}

pub const fn get_rune_bonus_stats(
    stats: &mut Stats,
    runes: &[RuneId],
    modifiers: &mut Modifiers,
    level: u8,
    adaptive_force: &mut f32,
) {
    stats.get_rune_bonus_stats(runes, modifiers, adaptive_force, level);
}

#[derive(Debug)]
pub struct InferStats<'a> {
    pub item_exceptions: &'a [ValueException],
    pub rune_exceptions: &'a [ValueException],
    pub items: &'a [ItemId],
    pub runes: &'a [RuneId],
    pub modifiers: &'a mut Modifiers,
    pub dragons: Dragons,
    pub ability_levels: AbilityLevels,
    pub stacks: u32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub is_mega_gnar: bool,
}

pub const fn infer_champion_stats(data: InferStats<'_>) -> Stats {
    let InferStats {
        item_exceptions,
        rune_exceptions,
        items,
        runes,
        modifiers,
        dragons,
        ability_levels,
        stacks,
        level,
        champion_id,
        is_mega_gnar,
    } = data;

    let mut bonus_stats = Stats::default();

    let data = champion_id.data();

    let mut adaptive_force = 0.0;

    bonus_stats.get_item_bonus_stats(items, modifiers, &mut adaptive_force);
    bonus_stats.get_rune_bonus_stats(runes, modifiers, &mut adaptive_force, level);

    let cached_stats = data.stats;
    let base_stats = champion_id.base_stats(level, is_mega_gnar);

    let mut stats = Stats {
        armor: base_stats.armor,
        attack_damage: base_stats.attack_damage,
        crit_damage: cached_stats.crit_base * cached_stats.crit_modifier,
        current_health: base_stats.max_health,
        magic_resist: base_stats.magic_resist,
        max_health: base_stats.max_health,
        max_mana: base_stats.max_mana,
        current_mana: base_stats.max_mana,
        ability_power: 0.0,
        attack_speed: 0.0,
        armor_penetration_flat: 0.0,
        armor_penetration_percent: 0.0,
        magic_penetration_flat: 0.0,
        magic_penetration_percent: 0.0,
        crit_chance: 0.0,
    };

    stats.ability_power += bonus_stats.ability_power;
    stats.armor += bonus_stats.armor;
    stats.armor_penetration_flat += bonus_stats.armor_penetration_flat;
    stats.armor_penetration_percent += bonus_stats.armor_penetration_percent;
    stats.attack_damage += bonus_stats.attack_damage;
    stats.attack_speed += bonus_stats.attack_speed;
    stats.crit_chance += bonus_stats.crit_chance;
    stats.crit_damage += bonus_stats.crit_damage;
    stats.current_health += bonus_stats.current_health;
    stats.magic_penetration_flat += bonus_stats.magic_penetration_flat;
    stats.magic_penetration_percent += bonus_stats.magic_penetration_percent;
    stats.magic_resist += bonus_stats.magic_resist;
    stats.max_health += bonus_stats.max_health;
    stats.max_mana += bonus_stats.max_mana;
    stats.current_mana += bonus_stats.current_mana;

    stats.assign_rune_exceptions(
        rune_exceptions,
        &mut adaptive_force,
        data.attack_type,
        level,
    );

    let adaptive_type =
        match AdaptiveType::try_infer(bonus_stats.attack_damage, bonus_stats.ability_power) {
            Some(x) => x,
            None => data.adaptive_type,
        };

    match adaptive_type {
        AdaptiveType::Magic => stats.ability_power += adaptive_force,
        AdaptiveType::Physical => stats.attack_damage += 0.6 * adaptive_force,
    }

    stats.assign_item_exceptions(item_exceptions);
    stats.assign_champion_exceptions(champion_id, ability_levels, stacks);

    stats.attack_speed = cached_stats.attack_speed(stats.attack_speed, level);

    let mut i = 0;
    while i < items.len() {
        match items[i] {
            ItemId::OverlordsBloodmail => {
                stats.attack_damage += 0.025 * bonus_stats.max_health;
            }
            ItemId::Riftmaker => {
                modifiers.damages.global_mod *= 1.08;
                stats.ability_power += 0.02 * (bonus_stats.max_health + stats.max_health);
            }
            ItemId::ArchangelsStaff => stats.ability_power += 0.01 * bonus_stats.max_mana,
            ItemId::SeraphsEmbrace => stats.ability_power += 0.02 * bonus_stats.max_mana,
            ItemId::DemonicEmbrace => {
                stats.ability_power += 0.02 * (bonus_stats.max_health + stats.max_health)
            }
            ItemId::Manamune | ItemId::Muramana => {
                stats.attack_damage += 0.025 * bonus_stats.max_mana;
            }
            ItemId::WintersApproach | ItemId::Fimbulwinter => {
                stats.max_health += 0.15 * bonus_stats.max_mana;
            }
            ItemId::JuiceOfVitality => stats.max_health += 300.0 + 0.1 * stats.max_health,
            ItemId::RabadonsDeathcap => stats.ability_power *= 1.3,
            ItemId::WoogletsWitchcap => stats.ability_power *= 1.5,
            ItemId::WarmogsArmor => stats.max_health *= 1.12,
            ItemId::JuiceOfPower => {
                stats.attack_damage += 18.0 + 0.1 * stats.attack_damage;
                stats.ability_power += 30.0 + 0.1 * stats.ability_power;
            }
            _ => {}
        }
        i += 1;
    }

    let fire = RiotFormulas::get_fire_multiplier(dragons.ally_fire_dragons);
    let earth = RiotFormulas::get_earth_multiplier(dragons.ally_earth_dragons);

    stats.ability_power *= fire;
    stats.attack_damage *= fire;
    stats.magic_resist *= earth;
    stats.armor *= earth;

    stats.crit_chance = stats.crit_chance.clamp(0.0, 100.0);

    stats
}

impl Stats {
    pub const fn infer(data: InferStats<'_>) -> Self {
        infer_champion_stats(data)
    }

    pub const fn get_item_bonus_stats(
        &mut self,
        items: &[ItemId],
        modifiers: &mut Modifiers,
        adaptive_force: &mut f32,
    ) {
        let mut armor_pen_mult = 1.0;
        let mut magic_pen_mult = 1.0;

        let mut i = 0;
        while i < items.len() {
            let item_id = items[i];
            let item = item_id.data();
            let item_stats = item.stats;

            let mut armor_pen = 0.0;
            let mut magic_pen = 0.0;

            let mut j = 0;
            while j < item_stats.len() {
                let (stat, value) = item_stats[j];
                let v = value as f32;

                match stat {
                    StatName::AbilityPower => self.ability_power += v,
                    StatName::AdaptiveForce => *adaptive_force += v,
                    StatName::Armor => self.armor += v,
                    StatName::ArmorPenetration => armor_pen = v,
                    StatName::AttackDamage => self.attack_damage += v,
                    StatName::AttackSpeed => self.attack_speed += v,
                    StatName::CritChance => self.crit_chance += v,
                    StatName::CritDamage => self.crit_damage += v,
                    StatName::Health => self.max_health += v,
                    StatName::Lethality => self.armor_penetration_flat += v,
                    StatName::MagicPenetration => self.magic_penetration_flat += v,
                    StatName::MagicPenetrationPercent => magic_pen = v,
                    StatName::MagicResist => self.magic_resist += v,
                    StatName::Mana => self.max_mana += v,
                    _ => {}
                }
                j += 1;
            }

            armor_pen_mult *= 1.0 - (armor_pen.clamp(0.0, 100.0) * 0.01);
            magic_pen_mult *= 1.0 - (magic_pen.clamp(0.0, 100.0) * 0.01);

            match item_id {
                ItemId::ElixirOfIron => self.max_health += 300.0,
                ItemId::Shadowflame => {
                    modifiers.damages.magic_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
                    modifiers.damages.true_mod *= RiotFormulas::SHADOWFLAME_BONUS_DAMAGE;
                }
                ItemId::SpearOfShojin => {
                    let AbilityModifiers { q, w, e, r } = &mut modifiers.abilities;
                    *q *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    *w *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    *e *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                    *r *= RiotFormulas::SHOJIN_BONUS_DAMAGE;
                }
                _ => {}
            }

            i += 1;
        }

        self.armor_penetration_percent = (1.0 - armor_pen_mult) * 100.0;
        self.magic_penetration_percent = (1.0 - magic_pen_mult) * 100.0;
    }

    pub const fn get_rune_bonus_stats(
        &mut self,
        runes: &[RuneId],
        modifiers: &mut Modifiers,
        adaptive_force: &mut f32,
        level: u8,
    ) {
        let mut i = 0;

        while i < runes.len() {
            match runes[i] {
                RuneId::Waterwalking => {
                    *adaptive_force += (12 + level) as f32;
                }
                RuneId::AbsoluteFocus => *adaptive_force += 1.8 + 16.2 / 17.0 * (level - 1) as f32,
                RuneId::CoupDeGrace | RuneId::CutDown => {
                    modifiers.damages.global_mod *=
                        RiotFormulas::COUP_DE_GRACE_AND_CUTDOWN_BONUS_DAMAGE
                }
                RuneId::LastStand => {
                    modifiers.damages.global_mod *= RiotFormulas::get_last_stand(
                        1.0 - (self.current_health / self.max_health.max(1.0)),
                    )
                }
                RuneId::AxiomArcanist => modifiers.abilities.r *= 1.12,
                RuneId::Health => self.max_health += 65.0,
                RuneId::HealthScaling => self.max_health += 10.0 * level as f32,
                _ => {}
            }
            i += 1;
        }
    }

    pub const fn assign_item_exceptions(&mut self, exceptions: &[ValueException]) {
        if exceptions.is_empty() {
            return;
        }

        let mut i = 0;
        while i < exceptions.len() {
            let item_exception = exceptions[i];
            let stacks = item_exception.stacks();

            if let Some(item_id) = item_exception.get_item_id() {
                match item_id {
                    ItemId::DarkSeal => self.ability_power += (stacks << 2) as f32,
                    ItemId::Dragonheart => {
                        let modifier = 1.0 + 0.04 * stacks as f32;
                        self.ability_power *= modifier;
                        self.attack_speed *= modifier;
                        self.attack_damage *= modifier;
                        self.max_health *= modifier;
                        self.armor *= modifier;
                        self.magic_resist *= modifier;
                    }
                    ItemId::DemonKingsCrown => {
                        let modifier = 1.0 + 0.01 * stacks as f32;
                        self.ability_power *= modifier;
                        self.attack_speed *= modifier;
                        self.attack_damage *= modifier;
                        self.max_health *= modifier;
                        self.armor *= modifier;
                        self.magic_resist *= modifier;
                    }
                    ItemId::RiteOfRuin => self.crit_chance += stacks as f32 * 2.5,
                    ItemId::MejaisSoulstealer => self.ability_power += (5 * stacks) as f32,
                    ItemId::BlackCleaver => {
                        self.armor_penetration_percent = RiotFormulas::combine_percentage(
                            self.armor_penetration_percent,
                            (6 * stacks) as f32,
                        )
                    }
                    ItemId::BloodlettersCurse => {
                        self.magic_penetration_percent = RiotFormulas::combine_percentage(
                            self.magic_penetration_percent,
                            7.5 * stacks as f32,
                        )
                    }
                    ItemId::Hubris => self.attack_damage += (15 + (stacks << 1)) as f32,
                    _ => {}
                }
            }
            i += 1;
        }
    }

    pub const fn assign_rune_exceptions(
        &mut self,
        exceptions: &[ValueException],
        adaptive_force: &mut f32,
        attack_type: AttackType,
        level: u8,
    ) {
        if exceptions.is_empty() {
            return;
        }

        let mut i = 0;
        while i < exceptions.len() {
            let rune_exception = exceptions[i];
            let stacks = rune_exception.stacks();
            if let Some(rune_id) = rune_exception.get_rune_id() {
                match rune_id {
                    RuneId::LethalTempo => match attack_type {
                        AttackType::Melee => {
                            self.attack_speed +=
                                (stacks as f32) * (5.0 + 11.0 / 17.0 * (level - 1) as f32);
                        }
                        AttackType::Ranged => {
                            self.attack_speed +=
                                (stacks as f32) * (3.6 + 4.4 / 17.0 * (level - 1) as f32);
                        }
                    },
                    RuneId::Conqueror => {
                        *adaptive_force += (stacks as f32) * (1.8 + 2.2 / 17.0 * (level - 1) as f32)
                    }
                    RuneId::EyeballCollection | RuneId::GhostPoro | RuneId::ZombieWard => {
                        *adaptive_force += match stacks {
                            ..10 => (stacks << 1) as f32,
                            10.. => 30.0,
                        }
                    }
                    RuneId::GatheringStorm => {
                        *adaptive_force += ((stacks * (stacks + 1)) << 2) as f32
                    }
                    RuneId::AdaptiveForce => *adaptive_force += 9.0 * stacks as f32,
                    RuneId::AttackSpeed => self.attack_speed += 10.0 * (stacks as f32),
                    _ => {}
                }
            }
            i += 1;
        }
    }

    pub const fn assign_champion_exceptions(
        &mut self,
        champion_id: ChampionId,
        ability_levels: AbilityLevels,
        stacks: u32,
    ) {
        match champion_id {
            ChampionId::Yasuo => self.attack_damage += 0.5 * (self.crit_chance - 100.0).max(0.0),
            ChampionId::Veigar => self.ability_power += stacks as f32,
            ChampionId::Swain => self.max_health += (12 * stacks) as f32,
            ChampionId::Chogath => {
                let mul = {
                    let r = ability_levels.r;
                    let min = 0;
                    let max = 3;
                    (if r < min {
                        min
                    } else if r > max {
                        max
                    } else {
                        r
                    }) as u32
                };
                self.max_health += (stacks * 80 + 40 * mul) as f32
            }
            ChampionId::Sion => self.max_health += stacks as f32,
            ChampionId::Darius => {
                self.armor_penetration_percent = RiotFormulas::combine_percentage(
                    self.armor_penetration_percent,
                    (15 + 5 * ability_levels.e) as f32,
                )
            }
            ChampionId::Pantheon => {
                self.armor_penetration_percent = RiotFormulas::combine_percentage(
                    self.armor_penetration_percent,
                    (10 * ability_levels.r) as f32,
                )
            }
            ChampionId::Nilah => {
                self.armor_penetration_percent = RiotFormulas::combine_percentage(
                    self.armor_penetration_percent,
                    self.crit_chance / 3.0,
                )
            }
            ChampionId::Mordekaiser => {
                self.magic_penetration_percent = RiotFormulas::combine_percentage(
                    self.magic_penetration_percent,
                    2.5 + 2.5 * ability_levels.e as f32,
                )
            }
            _ => {}
        };
    }
}

pub struct ChampionExceptionData<'a> {
    pub stats: &'a mut Stats,
    pub ability_levels: AbilityLevels,
    pub stacks: u32,
}

/// Receives basic information about the current player ability levels and its identifier,
/// and modifies the current stats based on the number of stacks. Note that depending on the
/// value of the enum [`ChampionId`], this function might do nothing
pub const fn assign_champion_exceptions(data: ChampionExceptionData, champion_id: ChampionId) {
    let ChampionExceptionData {
        stats,
        ability_levels,
        stacks,
    } = data;

    stats.assign_champion_exceptions(champion_id, ability_levels, stacks);
}

pub struct RuneExceptionData<'a> {
    pub adaptive_force: &'a mut f32,
    pub stats: &'a mut Stats,
    pub attack_type: AttackType,
    pub level: u8,
}

/// Receives mutable references to the champion's current stats, bonus stats and modifiers,
/// modifying their values based on the `exceptions` slice. Only the items that
/// depend on some stack count should be added to the match arms of this function
pub const fn assign_item_exceptions(stats: &mut Stats, exceptions: &[ValueException]) {
    stats.assign_item_exceptions(exceptions);
}

pub const fn assign_rune_exceptions(data: RuneExceptionData, exceptions: &[ValueException]) {
    let RuneExceptionData {
        adaptive_force,
        stats,
        attack_type,
        level,
    } = data;

    stats.assign_rune_exceptions(exceptions, adaptive_force, attack_type, level);
}

/// Receives data about some custom game, containing the minimum information about the
/// current player and the enemy players, returning a new struct containing the calculated
/// damages against several entities. This function is generally safe to use, but it assumes
/// that the received struct [`InputGame`] is valid. There's no undefined behavior checks.
pub fn calculator(game: InputGame<'_>) -> OutputGame {
    let InputGame {
        active_player:
            InputActivePlayer {
                abilities: ability_levels,
                runes: current_player_raw_runes,
                rune_exceptions,
                data:
                    InputMinData {
                        stats: champion_raw_stats,
                        level,
                        items: current_player_raw_items,
                        champion_id: current_player_champion_id,
                        is_mega_gnar,
                        stacks,
                        item_exceptions,
                    },
            },
        dragons,
        enemy_players,
    } = game;

    let mut modifiers = Modifiers::default();

    let current_player_cache = current_player_champion_id.data();
    let current_player_base_stats = current_player_champion_id.base_stats(level, is_mega_gnar);

    let champion_stats = champion_raw_stats.unwrap_or_else(|| {
        infer_champion_stats(InferStats {
            item_exceptions: &item_exceptions,
            rune_exceptions: &rune_exceptions,
            items: &current_player_raw_items,
            runes: &current_player_raw_runes,
            modifiers: &mut modifiers,
            dragons,
            ability_levels,
            stacks,
            level,
            champion_id: current_player_champion_id,
            is_mega_gnar,
        })
    });

    let current_player_bonus_stats = champion_stats.bonus_stats(current_player_base_stats);

    let adaptive_type = match AdaptiveType::try_infer(
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
    ) {
        Some(x) => x,
        None => current_player_cache.adaptive_type,
    };

    let shred = ResistShred::new(&champion_stats);
    let tower_damages = get_tower_damages(
        adaptive_type,
        current_player_base_stats.attack_damage,
        current_player_bonus_stats.attack_damage,
        champion_stats.ability_power,
        shred,
    );

    let current_player_runes = get_damaging_runes(&current_player_raw_runes);
    let current_player_items = get_damaging_items(&current_player_raw_items);

    let self_state = SelfState {
        stacks: stacks as _,
        current_stats: champion_stats,
        bonus_stats: current_player_bonus_stats,
        base_stats: current_player_base_stats,
        adaptive_type,
        ability_levels,
        level,
    };

    let attack_type = current_player_cache.attack_type;

    // Everything up to this point was const-evaluable. The functions below require
    // heap allocation, therefore they're not constant qualified

    let eval_data = DamageEvalData {
        abilities: StaticDamageKind {
            metadata: current_player_cache.metadata,
            closures: current_player_cache.closures,
        },
        items: DamageKind::items(&current_player_items, attack_type),
        runes: DamageKind::runes(&current_player_runes, attack_type),
    };

    let monster_damages = get_monster_damages(&self_state, &eval_data, shred);
    let enemies = get_calculator_enemies(
        enemy_players,
        &self_state,
        &eval_data,
        modifiers,
        shred,
        game.dragons.enemy_earth_dragons,
    );

    OutputGame {
        current_player: OutputCurrentPlayer {
            base_stats: current_player_base_stats,
            bonus_stats: current_player_bonus_stats,
            current_stats: champion_stats,
            champion_id: current_player_champion_id,
            adaptive_type,
            level,
        },
        items_meta: eval_data.items.metadata,
        runes_meta: eval_data.runes.metadata,
        monster_damages,
        tower_damages,
        enemies,
    }
}

pub fn get_calculator_enemies(
    enemy_players: &[InputMinData<'_, EnemyStats>],
    self_state: &SelfState,
    eval_data: &DamageEvalData,
    modifiers: Modifiers,
    shred: ResistShred,
    enemy_earth_dragons: u16,
) -> Box<[OutputEnemy]> {
    enemy_players
        .into_iter()
        .map(|player| {
            let InputMinData {
                items: e_items,
                stacks: e_stacks,
                stats: e_stats,
                level: e_level,
                champion_id: e_champion_id,
                is_mega_gnar: e_is_mega_gnar,
                item_exceptions: e_item_exceptions,
            } = *player;

            let e_base_stats = SimpleStats::base_stats(e_champion_id, e_level, e_is_mega_gnar);
            let full_state = EnemyState {
                current_stats: e_stats,
                base_stats: e_base_stats,
                items: &e_items,
                stacks: e_stacks,
                champion_id: e_champion_id,
                level: e_level,
                item_exceptions: &e_item_exceptions,
                earth_dragons: enemy_earth_dragons,
            }
            .full_state(shred, false);

            let ctx = full_state.ctx(self_state);

            let modifiers = Modifiers {
                damages: DamageModifiers {
                    adaptive_type: self_state.adaptive_type,
                    physical_mod: modifiers.damages.physical_mod
                        * full_state.armor_values.modifier
                        * full_state.modifiers.physical_mod,
                    magic_mod: modifiers.damages.magic_mod
                        * full_state.magic_values.modifier
                        * full_state.modifiers.magic_mod,
                    true_mod: modifiers.damages.true_mod * full_state.modifiers.true_mod,
                    global_mod: modifiers.damages.global_mod * full_state.modifiers.global_mod,
                },
                ..modifiers
            };

            // The only non-const function
            let damages = Damages::new(ctx, eval_data, modifiers);

            OutputEnemy {
                current_stats: full_state.current_stats,
                bonus_stats: full_state.bonus_stats,
                base_stats: e_base_stats,
                real_magic_resist: full_state.magic_values.real as _,
                real_armor: full_state.armor_values.real as _,
                champion_id: e_champion_id,
                level: e_level,
                damages,
            }
        })
        .collect::<Box<[OutputEnemy]>>()
}
