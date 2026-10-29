#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use tutorlolv2::{
    AttackType, ChampionId, ItemId, RuneId, calculator::calculator, helpers::get_stacks, model::*,
    realtime::realtime, riot::*,
};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

#[test]
fn purchase_and_context_helpers_are_const_callable() {
    const STATE: SelfState = SelfState {
        stacks: 0.0,
        ability_levels: AbilityLevels::default(),
        current_stats: PlayerStats {
            max_health: 2000.0,
            max_mana: 1000.0,
            ability_power: 100.0,
            ..PlayerStats::default()
        },
        base_stats: BasicStats {
            max_health: 1000.0,
            max_mana: 100.0,
            ..BasicStats::default()
        },
        bonus_stats: BasicStats {
            max_health: 1000.0,
            max_mana: 900.0,
            ..BasicStats::default()
        },
        level: NonZeroU8::MIN,
        adaptive_type: tutorlolv2::AdaptiveType::Physical,
    };
    const PURCHASE: SelfState = STATE.simulated_purchase(
        ItemId::SapphireCrystal,
        &[ItemId::WarmogsArmor, ItemId::Fimbulwinter],
        ChampionId::Gnar,
        Dragons::default(),
    );
    const ENEMY: EnemyFullState = EnemyFullState {
        current_stats: EnemyStats {
            armor: 100.0,
            current_health: 1000.0,
            magic_resist: 50.0,
            max_health: 1000.0,
            missing_health: 1.0,
        },
        bonus_stats: SimpleStats::default(),
        modifiers: DamageModifiers::default(),
        armor_values: ResistValue {
            real: 100.0,
            modifier: 0.5,
        },
        magic_values: ResistValue {
            real: 50.0,
            modifier: 2.0 / 3.0,
        },
        steelcaps: false,
        rocksolid: false,
        randuin: false,
    };
    const SELF_CTX: tutorlolv2::Ctx = PURCHASE.ctx(&ENEMY);
    const ENEMY_CTX: tutorlolv2::Ctx = ENEMY.ctx(&PURCHASE);
    close(SELF_CTX.max_health, 2050.4);
    close(SELF_CTX.bonus_health, 1050.4);
    assert_eq!(SELF_CTX, ENEMY_CTX);
}

#[test]
fn damage_sum_is_const_callable_without_changing_aggregation() {
    const fn sum(damage: &Damages) -> i32 {
        damage.sum()
    }
    let damage = Damages {
        attacks: Attacks {
            onhit_damage: RangeDamage {
                minimum_damage: 2,
                maximum_damage: 4,
            },
            ..Default::default()
        },
        abilities: vec![3, 7].into_boxed_slice(),
        items: vec![5].into_boxed_slice(),
        runes: vec![11].into_boxed_slice(),
        ..Default::default()
    };
    assert_eq!(sum(&damage), 32);
    assert_eq!(sum(&Damages::default()), 0);
}

#[test]
fn ornn_upgrade_estimate_preserves_baseline() {
    for (level, multiplier) in [(12, 1.10), (13, 1.14), (16, 1.26), (17, 1.30), (18, 1.30)] {
        let state = EnemyState {
            champion_id: ChampionId::Ornn,
            level: NonZeroU8::new(level).unwrap(),
            base_stats: SimpleStats {
                armor: 100.0,
                magic_resist: 100.0,
                max_health: 1000.0,
            },
            ..Default::default()
        }
        .full_state(ResistShred::new(&PlayerStats::default()), false);
        close(state.current_stats.armor, 100.0 * multiplier);
        close(state.current_stats.magic_resist, 100.0 * multiplier);
        close(state.current_stats.max_health, 1000.0 * multiplier);
    }
}

#[test]
fn supplied_enemy_stats_skip_inference_in_const_context() {
    const SUPPLIED: EnemyStats = EnemyStats {
        armor: 120.0,
        magic_resist: 80.0,
        max_health: 2000.0,
        current_health: 1500.0,
        missing_health: 99.0,
    };
    // Swain's inferred 12 * stacks would overflow at compile time if evaluated.
    const FULL: EnemyFullState = EnemyState {
        current_stats: Some(SUPPLIED),
        base_stats: SimpleStats {
            armor: 20.0,
            magic_resist: 30.0,
            max_health: 1000.0,
        },
        items: &[ItemId::WarmogsArmor, ItemId::FrozenHeart],
        item_exceptions: &[ValueException::pack_item_id(ItemId::WarmogsArmor, 0)],
        champion_id: ChampionId::Swain,
        stacks: u32::MAX,
        earth_dragons: 3,
        level: NonZeroU8::MIN,
    }
    .full_state(
        ResistShred {
            armor_penetration_flat: 10.0,
            armor_penetration_percent: 25.0,
            magic_penetration_flat: 5.0,
            magic_penetration_percent: 50.0,
        },
        false,
    );
    assert_eq!(
        FULL.current_stats,
        EnemyStats {
            missing_health: 0.25,
            ..SUPPLIED
        }
    );
    assert_eq!(
        FULL.bonus_stats,
        SimpleStats {
            armor: 100.0,
            magic_resist: 50.0,
            max_health: 1000.0,
        }
    );
    close(FULL.armor_values.real, 80.0);
    close(FULL.magic_values.real, 35.0);
    const { assert!(FULL.rocksolid) };
}

#[test]
fn supplied_and_inferred_enemies_keep_defensive_effects_and_item_flags() {
    for (items, steelcaps, rocksolid, randuin) in [
        (&[][..], false, false, false),
        (&[ItemId::PlatedSteelcaps][..], true, false, false),
        (&[ItemId::ArmoredAdvance][..], true, false, false),
        (&[ItemId::FrozenHeart][..], false, true, false),
        (&[ItemId::WardensMail][..], false, true, false),
        (&[ItemId::RanduinsOmen][..], false, true, true),
        (
            &[ItemId::PlatedSteelcaps, ItemId::RanduinsOmen][..],
            true,
            true,
            true,
        ),
    ] {
        let enemy = EnemyState {
            champion_id: ChampionId::Kassadin,
            base_stats: SimpleStats {
                armor: 40.0,
                magic_resist: 30.0,
                max_health: 1000.0,
            },
            items,
            earth_dragons: 2,
            ..Default::default()
        };
        let shred = ResistShred::default();
        let inferred = enemy.full_state(shred, false);
        let supplied = EnemyState {
            current_stats: Some(inferred.current_stats),
            ..enemy
        }
        .full_state(shred, false);
        for state in [inferred, supplied] {
            close(state.modifiers.magic_mod, 0.9);
            assert_eq!(
                (state.steelcaps, state.rocksolid, state.randuin),
                (steelcaps, rocksolid, randuin)
            );
        }
        assert_eq!(supplied.armor_values, inferred.armor_values);
        assert_eq!(supplied.magic_values, inferred.magic_values);
        assert_eq!(supplied.bonus_stats, inferred.bonus_stats);
    }
}

#[test]
fn estimated_stacks_grow_with_elapsed_time() {
    for (champion, average) in [
        (ChampionId::Nasus, 450.0),
        (ChampionId::AurelionSol, 190.0),
        (ChampionId::Kindred, 5.0),
    ] {
        close(get_stacks(champion, 900.0), average / 2.0);
        close(get_stacks(champion, 1800.0), average);
        close(get_stacks(champion, 3600.0), average * 2.0);
    }
}

#[test]
fn simulated_dragons_only_multiply_new_item_stats() {
    let stats = PlayerStats {
        attack_damage: 200.0,
        ability_power: 300.0,
        armor: 100.0,
        magic_resist: 90.0,
        ..PlayerStats::default()
    };
    let plain = stats.get_simulated_stats(Dragons::default());
    let buffed = stats.get_simulated_stats(Dragons {
        ally_fire_dragons: 2,
        ally_earth_dragons: 2,
        ..Dragons::default()
    });
    for (plain, buffed) in plain.iter().zip(&buffed) {
        close(
            buffed.attack_damage,
            200.0 + (plain.attack_damage - 200.0) * 1.06,
        );
        close(
            buffed.ability_power,
            300.0 + (plain.ability_power - 300.0) * 1.06,
        );
        close(buffed.armor, 100.0 + (plain.armor - 100.0) * 1.10);
        close(
            buffed.magic_resist,
            90.0 + (plain.magic_resist - 90.0) * 1.10,
        );
    }
}

fn game<'a>(
    items: &'a [ItemId],
    runes: &'a [RuneId],
    enemies: &'a [InputMinData<'a, EnemyStats>],
) -> InputGame<'a> {
    InputGame {
        active_player: InputActivePlayer {
            data: InputMinData {
                champion_id: ChampionId::Gnar,
                level: NonZeroU8::new(18).unwrap(),
                items,
                ..Default::default()
            },
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            runes,
            ..Default::default()
        },
        enemy_players: enemies,
        ..Default::default()
    }
}

#[test]
fn explicit_stats_keep_item_and_rune_damage_modifiers() {
    let enemies = [InputMinData {
        champion_id: ChampionId::Aatrox,
        level: NonZeroU8::new(18).unwrap(),
        ..Default::default()
    }];
    let mut input = game(
        &[
            ItemId::SpearOfShojin,
            ItemId::Shadowflame,
            ItemId::Riftmaker,
        ],
        &[
            RuneId::AxiomArcanist,
            RuneId::CoupDeGrace,
            RuneId::LastStand,
        ],
        &enemies,
    );
    let inferred = calculator(input);
    input.active_player.data.stats = Some(inferred.current_player.current_stats);
    let explicit = calculator(input);
    assert_eq!(explicit.enemies[0].damages, inferred.enemies[0].damages);
    assert_eq!(explicit.monster_damages, inferred.monster_damages);
}

#[test]
fn mega_gnar_uses_melee_item_damage() {
    let enemies = [InputMinData {
        champion_id: ChampionId::Aatrox,
        level: NonZeroU8::new(18).unwrap(),
        ..Default::default()
    }];
    let mut input = game(&[ItemId::BladeOfTheRuinedKing], &[], &enemies);
    input.active_player.data.is_mega_gnar = true;
    let output = calculator(input);
    let damage = &output.enemies[0].damages;
    let raw = ItemId::BladeOfTheRuinedKing.eval(&damage.ctx, AttackType::Melee);
    let modifier = damage.ctx.physical_multiplier;
    assert_eq!(damage.items[0], (raw[0] * modifier) as i32);
}

fn live_game() -> RiotRealtime<'static> {
    let stats = PlayerStats {
        attack_damage: 200.0,
        ability_power: 100.0,
        armor: 80.0,
        magic_resist: 60.0,
        max_health: 2000.0,
        current_health: 2000.0,
        max_mana: 1000.0,
        current_mana: 1000.0,
        attack_speed: 1.0,
        crit_damage: 200.0,
        // Live API represents percent penetration as the remaining fraction.
        armor_penetration_percent: 1.0,
        magic_penetration_percent: 1.0,
        ..PlayerStats::default()
    };
    let player = |name, id, team| RiotAllPlayers {
        champion_name: name,
        raw_champion_name: "",
        raw_skin_name: "",
        riot_id: id,
        team,
        position: "TOP",
        level: NonZeroU8::new(18).unwrap(),
        items: Box::new([]),
        scores: RiotScoreboard {
            kills: 0,
            deaths: 0,
            assists: 0,
            creep_score: 0,
        },
    };
    RiotRealtime {
        active_player: RiotActivePlayer {
            abilities: RiotAbilities {
                q: RiotAbility { ability_level: 5 },
                w: RiotAbility { ability_level: 5 },
                e: RiotAbility { ability_level: 5 },
                r: RiotAbility { ability_level: 3 },
            },
            champion_stats: stats,
            full_runes: RiotFullRunes {
                general_runes: None,
            },
            level: NonZeroU8::new(18).unwrap(),
            riot_id: "self",
        },
        all_players: vec![
            player("Gnar", "self", "ORDER"),
            player("Aatrox", "enemy", "CHAOS"),
        ]
        .into_boxed_slice(),
        events: RiotRealtimeEvents {
            events: Box::new([]),
        },
        game_data: RiotRealtimeGameData {
            game_time: 1800.0,
            map_number: 11,
        },
    }
}

fn sim_index(item: ItemId) -> usize {
    ItemId::SIML
        .iter()
        .position(|&id| id == item)
        .expect("fixture item must be simulated")
}

#[test]
fn simulated_ad_updates_bonus_scaling() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let original = &out.enemies[0].damages.ctx;
    let sim = &out.enemies[0].siml_items[sim_index(ItemId::SpearOfShojin)].ctx;
    assert!(sim.attack_damage > original.attack_damage);
    close(
        sim.bonus_ad - original.bonus_ad,
        sim.attack_damage - original.attack_damage,
    );
}

#[test]
fn simulated_penetration_changes_resistance_multipliers() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let enemy = &out.enemies[0];
    let armor_sim = &enemy.siml_items[sim_index(ItemId::YoumuusGhostblade)].ctx;
    let magic_sim = &enemy.siml_items[sim_index(ItemId::VoidStaff)].ctx;
    assert!(armor_sim.physical_multiplier > enemy.damages.ctx.physical_multiplier);
    assert!(magic_sim.magic_multiplier > enemy.damages.ctx.magic_multiplier);
    close(
        armor_sim.physical_multiplier,
        100.0 / (100.0 + (enemy.current_stats.armor - armor_sim.armor_penetration_flat).max(0.0)),
    );
    close(
        magic_sim.magic_multiplier,
        100.0
            / (100.0
                + enemy.current_stats.magic_resist
                    * (1.0 - magic_sim.magic_penetration_percent / 100.0)),
    );
}

#[test]
fn simulated_purchase_adds_item_damage() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let sim = &out.enemies[0].siml_items[sim_index(ItemId::NashorsTooth)];
    assert_eq!(sim.items.len(), 2);
    assert!(sim.items[0] > 0);
}

#[test]
fn simulated_purchase_applies_ability_modifiers() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let enemy = &out.enemies[0];
    let sim = &enemy.siml_items[sim_index(ItemId::SpearOfShojin)];
    let without_buff = tutorlolv2::model::Damages::eval_abilities(
        &sim.ctx,
        &mut RangeDamage::default(),
        Modifiers::new(&sim.ctx, out.current_player.adaptive_type),
        ChampionId::Gnar.metadata(),
        ChampionId::Gnar.data().closures,
    );
    assert!(
        sim.abilities
            .iter()
            .zip(&without_buff)
            .any(|(actual, plain)| actual > plain)
    );
}

#[test]
fn level_255_with_waterwalking_does_not_overflow() {
    let mut input = game(&[], &[RuneId::Waterwalking], &[]);
    input.active_player.data.level = NonZeroU8::new(255).unwrap();
    let out = calculator(input);
    assert_eq!(out.current_player.level.get(), 255);
    assert!(out.current_player.current_stats.attack_damage.is_finite());
}

#[test]
fn simulated_purchase_applies_stat_conversion_and_attack_speed_units() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let enemy = &out.enemies[0];
    // Deathcap is currently excluded by generated SIML tier filtering; exercise
    // the same purchase helper directly rather than changing that item selection.
    let player = &out.current_player;
    let state = SelfState {
        current_stats: player.current_stats,
        bonus_stats: player.bonus_stats,
        base_stats: player.base_stats,
        adaptive_type: player.adaptive_type,
        ability_levels: out.ability_levels,
        level: player.level,
        stacks: 0.0,
    };
    let cap = state.simulated_purchase(
        ItemId::RabadonsDeathcap,
        &[],
        player.champion_id,
        out.dragons,
    );
    close(cap.current_stats.ability_power, 299.0); // (100 live AP + 130 item AP) * 1.3
    let warmog =
        state.simulated_purchase(ItemId::WarmogsArmor, &[], player.champion_id, out.dragons);
    close(warmog.current_stats.max_health, 3360.0); // (2000 live HP + 1000 item HP) * 1.12
    let nashor = &enemy.siml_items[sim_index(ItemId::NashorsTooth)];
    close(nashor.ctx.attack_speed, 1.3125); // 1 live AS + .625 ratio * 50%
    assert_eq!(
        nashor.attacks.onhit_damage.minimum_damage,
        nashor.attacks.basic_attack + nashor.items[0]
    );
}

#[test]
fn candidate_pair_is_always_appended_without_duplicating_owned_effects() {
    let mut input = live_game();
    input.all_players[0].items = vec![RiotItems { item_id: 3115 }].into_boxed_slice();
    let out = realtime(&input).unwrap();
    assert_eq!(out.items_meta[0].kind, ItemId::NashorsTooth);
    let owned_sim = &out.enemies[0].siml_items[sim_index(ItemId::NashorsTooth)];
    assert_eq!(owned_sim.items.len(), 4);
    assert_eq!(&owned_sim.items[2..], &[0, 0]);
    assert_eq!(
        owned_sim.attacks.onhit_damage.minimum_damage,
        owned_sim.attacks.basic_attack + owned_sim.items[0]
    );
    let new_sim = &out.enemies[0].siml_items[sim_index(ItemId::BladeOfTheRuinedKing)];
    assert_eq!(new_sim.items.len(), 4);
    assert!(new_sim.items[0] > 0);
    assert!(new_sim.items[2] > 0);
}

#[test]
fn every_simulated_purchase_has_one_extra_item_pair() {
    for owned in [None, Some(ItemId::NashorsTooth)] {
        let mut input = live_game();
        if let Some(item) = owned {
            input.all_players[0].items = vec![RiotItems {
                item_id: item.riot_id(),
            }]
            .into_boxed_slice();
        }
        let out = realtime(&input).unwrap();
        let base_len = out.items_meta.len() * 2;
        for enemy in &out.enemies {
            assert_eq!(enemy.damages.items.len(), base_len);
            for (candidate, damage) in ItemId::SIML.iter().zip(&enemy.siml_items) {
                assert_eq!(damage.items.len(), base_len + 2, "{candidate:?}");
                if !candidate.deals_damage() || Some(*candidate) == owned {
                    assert_eq!(&damage.items[base_len..], &[0, 0], "{candidate:?}");
                }
                assert_eq!(damage.runes.len(), out.runes_meta.len() * 2);
            }
        }
    }
}

#[test]
fn backend_decoding_rejects_zero_level_but_accepts_255() {
    let config = bincode::config::standard();
    // OwnedInputMinData's wire order, with a raw byte for an invalid level.
    let zero = (
        None::<PlayerStats>,
        Vec::<ItemId>::new(),
        Vec::<ValueException>::new(),
        0u32,
        0u8,
        false,
        ChampionId::Gnar,
    );
    let bytes = bincode::encode_to_vec(zero, config).unwrap();
    assert!(
        bincode::decode_from_slice::<OwnedInputMinData<PlayerStats>, _>(&bytes, config).is_err()
    );
    let input = OwnedInputMinData::<PlayerStats> {
        level: NonZeroU8::MAX,
        ..Default::default()
    };
    let bytes = bincode::encode_to_vec(&input, config).unwrap();
    let (decoded, used) =
        bincode::decode_from_slice::<OwnedInputMinData<PlayerStats>, _>(&bytes, config).unwrap();
    assert_eq!(decoded.level.get(), 255);
    assert_eq!(used, bytes.len());
}

#[test]
fn all_champions_accept_level_boundaries_without_upper_clamping() {
    use strum::VariantArray;
    for &champion in ChampionId::VARIANTS {
        for level in [NonZeroU8::MIN, NonZeroU8::MAX] {
            let mut input = game(
                &[],
                &[
                    RuneId::Waterwalking,
                    RuneId::AbsoluteFocus,
                    RuneId::HealthScaling,
                ],
                &[],
            );
            input.active_player.data.champion_id = champion;
            input.active_player.data.level = level;
            let out = calculator(input);
            assert_eq!(out.current_player.level, level);
        }
    }
}

#[test]
fn mega_gnar_uses_melee_rune_stat_scaling() {
    let mut input = game(&[], &[RuneId::LethalTempo], &[]);
    let exceptions = [ValueException::pack_rune_id(RuneId::LethalTempo, 6)];
    input.active_player.rune_exceptions = &exceptions;
    let mini = calculator(input);
    input.active_player.data.is_mega_gnar = true;
    let mega = calculator(input);
    // At level 18: 6 * 16% melee versus 6 * 8% ranged, .625 AS ratio.
    close(
        mega.current_player.current_stats.attack_speed
            - mini.current_player.current_stats.attack_speed,
        0.3,
    );
}

#[test]
fn health_scaling_shard_stacks_when_taken_twice() {
    let mut input = game(&[], &[RuneId::HealthScaling], &[]);
    input.active_player.data.level = NonZeroU8::new(18).unwrap();
    let once = [ValueException::pack_rune_id(RuneId::HealthScaling, 1)];
    input.active_player.rune_exceptions = &once;
    let single = calculator(input);
    let twice = [ValueException::pack_rune_id(RuneId::HealthScaling, 2)];
    input.active_player.rune_exceptions = &twice;
    let double = calculator(input);
    // Each shard grants 10 health per level.
    close(
        double.current_player.current_stats.max_health
            - single.current_player.current_stats.max_health,
        180.0,
    );
}

#[test]
fn const_evaluation_preserves_modifiers_with_explicit_stats() {
    use tutorlolv2::const_eval::{ConstEnemy, ConstInput};
    const INPUT: ConstInput<1, 1, 0, 0, 0, 0> = ConstInput {
        champion_id: ChampionId::Gnar,
        items: [ItemId::SpearOfShojin],
        runes: [RuneId::AxiomArcanist],
        rune_exceptions: [],
        item_exceptions: [],
        ability_levels: AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        },
        stats: None,
        dragons: Dragons::default(),
        stacks: 0,
        level: NonZeroU8::new(18).unwrap(),
        is_mega_gnar: true,
        enemy: ConstEnemy {
            champion_id: ChampionId::Aatrox,
            items: [],
            item_exceptions: [],
            stats: None,
            stacks: 0,
            level: NonZeroU8::new(18).unwrap(),
            is_mega_gnar: false,
        },
    };
    const INFERRED: tutorlolv2::const_eval::ConstOutput<
        { ChampionId::Gnar.number_of_abilities() },
        1,
        1,
    > = INPUT.eval();
    const EXPLICIT: tutorlolv2::const_eval::ConstOutput<
        { ChampionId::Gnar.number_of_abilities() },
        1,
        1,
    > = ConstInput {
        stats: Some(INFERRED.stats),
        ..INPUT
    }
    .eval();
    assert_eq!(EXPLICIT.damages.abilities, INFERRED.damages.abilities);
}

#[test]
fn owned_warmog_amplifies_health_converted_from_new_mana() {
    let state = SelfState {
        stacks: 0.0,
        ability_levels: AbilityLevels::default(),
        current_stats: PlayerStats {
            max_health: 2000.0,
            max_mana: 1000.0,
            ..PlayerStats::default()
        },
        base_stats: BasicStats {
            max_health: 1000.0,
            max_mana: 100.0,
            ..BasicStats::default()
        },
        bonus_stats: BasicStats {
            max_health: 1000.0,
            max_mana: 900.0,
            ..BasicStats::default()
        },
        level: NonZeroU8::new(18).unwrap(),
        adaptive_type: tutorlolv2::AdaptiveType::Physical,
    };
    for owned in [
        &[ItemId::WarmogsArmor, ItemId::Fimbulwinter][..],
        &[ItemId::Fimbulwinter, ItemId::WarmogsArmor][..],
    ] {
        let sim = state.simulated_purchase(
            ItemId::SapphireCrystal,
            owned,
            ChampionId::Gnar,
            Dragons::default(),
        );
        close(sim.current_stats.max_health, 2050.4); // 2000 + 300 * .15 * 1.12
    }
    let sim = state.simulated_purchase(
        ItemId::Fimbulwinter,
        &[ItemId::WarmogsArmor],
        ChampionId::Gnar,
        Dragons::default(),
    );
    close(sim.current_stats.max_health, 2935.2); // 2000 + (550 + 1900 * .15) * 1.12
}
