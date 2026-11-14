#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use math::{
    AttackType, ChampionId, GameMap, ItemId, L_MSTR, RuneId, calculator::calculator, model::*,
    realtime::realtime, riot::*,
};

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

#[test]
fn purchase_and_context_helpers_are_const_callable() {
    const STATE: SelfState = SelfState {
        bonus_attack_speed_ratio: types::AttackSpeedRatio::new(0.0),
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
        level: NonZeroU8::MIN,
        adaptive_type: math::AdaptiveType::Physical,
    };
    const PURCHASE: SelfState = STATE.simulated_purchase(
        ItemId::SapphireCrystal,
        &[ItemId::WarmogsArmor, ItemId::Fimbulwinter],
        ChampionId::Gnar,
        Dragons::default(),
        GameMap::SummonersRift,
    );
    const ENEMY: EnemyFullState = EnemyFullState {
        current_stats: EnemyStats {
            armor: 100.0,
            current_health: 1000.0,
            magic_resist: 50.0,
            max_health: 1000.0,
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
        accept_negatives: false,
    };
    const SELF_CTX: math::Ctx = PURCHASE.ctx(&ENEMY, GameMap::SummonersRift);
    const ENEMY_CTX: math::Ctx = ENEMY.ctx(&PURCHASE, GameMap::SummonersRift);
    close(SELF_CTX.max_health, 2050.4);
    close(SELF_CTX.bonus_health(), 1050.4);
    assert_eq!(SELF_CTX, ENEMY_CTX);
}

/// Inferred enemy stats for `champion_id` holding `items`, without any
/// penetration so the totals can be compared against a control champion
fn inferred_stats(champion_id: ChampionId, level: u8, stacks: u32, items: &[ItemId]) -> EnemyStats {
    EnemyState {
        champion_id,
        level: NonZeroU8::new(level).unwrap(),
        stacks,
        items,
        base_stats: SimpleStats {
            armor: 100.0,
            magic_resist: 100.0,
            max_health: 1000.0,
        },
        ..Default::default()
    }
    .full_state(ResistShred::new(&PlayerStats::default()), false)
    .current_stats
}

#[test]
fn ornn_upgrades_scale_only_bonus_stats() {
    const ITEMS: &[ItemId] = &[ItemId::SunfireAegis, ItemId::ForceOfNature];

    for (level, multiplier) in [(12, 1.10), (13, 1.14), (16, 1.26), (17, 1.30), (18, 1.30)] {
        // Garen has no stat special case, so it holds the same bonus stats unscaled
        let control = inferred_stats(ChampionId::Garen, level, 0, ITEMS);
        let ornn = inferred_stats(ChampionId::Ornn, level, 0, ITEMS);

        assert!(control.armor > 100.0 && control.magic_resist > 100.0);
        close(ornn.armor, 100.0 + (control.armor - 100.0) * multiplier);
        close(
            ornn.magic_resist,
            100.0 + (control.magic_resist - 100.0) * multiplier,
        );
        close(
            ornn.max_health - 1000.0,
            (control.max_health - 1000.0) * multiplier,
        );
    }

    // Without bonus stats there is nothing to upgrade
    let bare = inferred_stats(ChampionId::Ornn, 18, 0, &[]);
    let control = inferred_stats(ChampionId::Garen, 18, 0, &[]);
    close(bare.armor, control.armor);
    close(bare.max_health, control.max_health);
}

#[test]
fn chogath_feast_health_per_stack_follows_r_rank() {
    for (level, per_stack) in [(5, 0.0), (6, 80.0), (10, 80.0), (11, 120.0), (16, 160.0)] {
        let control = inferred_stats(ChampionId::Garen, level, 10, &[]);
        let chogath = inferred_stats(ChampionId::Chogath, level, 10, &[]);

        close(chogath.max_health - control.max_health, 10.0 * per_stack);
    }
}

#[test]
fn swain_soul_fragments_grant_health_per_stack() {
    // Ashe is a ranged control without stat special cases
    let control = inferred_stats(ChampionId::Ashe, 11, 10, &[]);
    let swain = inferred_stats(ChampionId::Swain, 11, 10, &[]);

    close(swain.max_health - control.max_health, 150.0);
}

#[test]
fn last_stand_only_applies_below_60_percent_health() {
    for (missing, expected) in [
        (0.0, 1.0),
        (0.2, 1.0),
        (0.39, 1.0),
        (0.4, 1.05),
        (0.55, 1.08),
        (0.7, 1.11),
        (1.0, 1.11),
    ] {
        close(RiotFormulas::get_last_stand(missing), expected);
    }
}

#[test]
fn supplied_enemy_stats_skip_inference_in_const_context() {
    const SUPPLIED: EnemyStats = EnemyStats {
        armor: 120.0,
        magic_resist: 80.0,
        max_health: 2000.0,
        current_health: 1500.0,
    };
    // Swain's inferred 12 * stacks would overflow at compile time if evaluated.
    const FULL: EnemyFullState = EnemyState {
        current_stats: Some(SUPPLIED),
        base_stats: SimpleStats {
            armor: 20.0,
            magic_resist: 30.0,
            max_health: 1000.0,
        },
        items: &[ItemId::WarmogsArmor, ItemId::WardensMail],
        item_exceptions: &[ValueException::pack_item_id(ItemId::WarmogsArmor, 0)],
        champion_id: ChampionId::Swain,
        stacks: u32::MAX,
        earth_dragons: 3,
        level: NonZeroU8::MIN,
        game_map: GameMap::Unknown,
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
    assert_eq!(FULL.current_stats, SUPPLIED);
    assert_eq!(FULL.current_stats.missing_health_ratio(), 0.25);
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
        // Only Warden's Mail still has the Rock Solid passive
        (&[ItemId::FrozenHeart][..], false, false, false),
        (&[ItemId::WardensMail][..], false, true, false),
        (&[ItemId::RanduinsOmen][..], false, false, true),
        (
            &[ItemId::PlatedSteelcaps, ItemId::RanduinsOmen][..],
            true,
            false,
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
        (ChampionId::Nasus, 600.0),
        (ChampionId::AurelionSol, 190.0),
        (ChampionId::Kindred, 5.0),
    ] {
        close(champion.avg_stacks(900.0), average / 2.0);
        close(champion.avg_stacks(1800.0), average);
        close(champion.avg_stacks(3600.0), average * 2.0);
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
    let plain = stats.get_simulated_stats(Dragons::default(), GameMap::SummonersRift);
    let buffed = stats.get_simulated_stats(
        Dragons {
            ally_fire_dragons: 2,
            ally_earth_dragons: 2,
            ..Dragons::default()
        },
        GameMap::SummonersRift,
    );
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

    assert_eq!(explicit.enemy_damages(0), inferred.enemy_damages(0));

    for m in 0..L_MSTR {
        assert_eq!(
            explicit.monster_damages(m),
            inferred.monster_damages(m),
            "monster row {m}"
        );
    }
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
    let damage = &&output.enemy_damages(0);
    let raw = ItemId::BladeOfTheRuinedKing.eval(&damage.ctx, AttackType::Melee);
    let modifier = damage.ctx.physical_multiplier();
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

    let original = out.enemy_damages(0).ctx;
    let sim = out.siml_damages(0, sim_index(ItemId::SpearOfShojin)).ctx;

    assert!(sim.attack_damage > original.attack_damage);
    close(
        sim.bonus_ad() - original.bonus_ad(),
        sim.attack_damage - original.attack_damage,
    );
}

#[test]
fn simulated_penetration_changes_resistance_multipliers() {
    let input = live_game();
    let out = realtime(&input).unwrap();

    let enemy = &out.enemies[0];
    let base = out.enemy_damages(0).ctx;
    let armor_sim = out
        .siml_damages(0, sim_index(ItemId::YoumuusGhostblade))
        .ctx;
    let magic_sim = out.siml_damages(0, sim_index(ItemId::VoidStaff)).ctx;

    assert!(armor_sim.physical_multiplier() > base.physical_multiplier());
    assert!(magic_sim.magic_multiplier() > base.magic_multiplier());
    close(
        armor_sim.physical_multiplier(),
        100.0 / (100.0 + (enemy.current_stats.armor - armor_sim.armor_penetration_flat).max(0.0)),
    );
    close(
        magic_sim.magic_multiplier(),
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
    let sim = &out.siml_damages(0, sim_index(ItemId::NashorsTooth));
    assert_eq!(sim.items.len(), 2);
    assert!(sim.items[0] > 0);
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
fn simulated_purchase_applies_ability_modifiers() {
    let input = live_game();
    let out = realtime(&input).unwrap();
    let sim = out.siml_damages(0, sim_index(ItemId::SpearOfShojin));

    let metadata = ChampionId::Gnar.metadata();
    let mut without_buff = vec![0; metadata.len()];

    math::model::DamageTable::eval_abilities(
        &sim.ctx,
        &mut RangeDamage::default(),
        Modifiers::new(&sim.ctx, out.current_player.adaptive_type),
        metadata,
        ChampionId::Gnar.data().closures,
        &mut without_buff,
    );

    assert!(
        sim.abilities
            .iter()
            .zip(&without_buff)
            .any(|(actual, plain)| actual > plain)
    );
}

#[test]
fn simulated_purchase_applies_stat_conversion_and_attack_speed_units() {
    let input = live_game();
    let out = realtime(&input).unwrap();

    // Deathcap is currently excluded by generated SIML tier filtering; exercise
    // the same purchase helper directly rather than changing that item selection.
    let player = &out.current_player;
    let state = SelfState {
        bonus_attack_speed_ratio: types::AttackSpeedRatio::new(0.0),
        current_stats: player.current_stats,
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
        GameMap::SummonersRift,
    );
    close(cap.current_stats.ability_power, 299.0); // (100 live AP + 130 item AP) * 1.3

    let warmog = state.simulated_purchase(
        ItemId::WarmogsArmor,
        &[],
        player.champion_id,
        out.dragons,
        GameMap::SummonersRift,
    );
    // Warmog's Vitality grants 12% of the bonus health, not of the base health
    close(
        warmog.current_stats.max_health,
        3000.0 + 0.12 * (3000.0 - player.base_stats.max_health),
    ); // 2000 live HP + 1000 item HP

    let nashor = out.siml_damages(0, sim_index(ItemId::NashorsTooth));
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

    let owned_sim = out.siml_damages(0, sim_index(ItemId::NashorsTooth));
    assert_eq!(owned_sim.items.len(), 4);
    assert_eq!(&owned_sim.items[2..], &[0, 0]);
    assert_eq!(
        owned_sim.attacks.onhit_damage.minimum_damage,
        owned_sim.attacks.basic_attack + owned_sim.items[0]
    );

    let new_sim = out.siml_damages(0, sim_index(ItemId::BladeOfTheRuinedKing));
    assert_eq!(new_sim.items.len(), 4);
    assert!(new_sim.items[0] > 0);
    assert!(new_sim.items[2] > 0);
}

#[test]
fn every_row_has_one_extra_item_pair() {
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
        let runes_len = out.runes_meta.len() * 2;

        assert_eq!(
            out.damages.rows(),
            out.enemies.len() * Realtime::ROWS_PER_ENEMY
        );

        for e in 0..out.enemies.len() {
            // Uniform stride: the base row also reserves the pair, always zero
            let base = out.enemy_damages(e);
            assert_eq!(base.items.len(), base_len + 2);
            assert_eq!(&base.items[base_len..], &[0, 0]);
            assert_eq!(base.runes.len(), runes_len);

            for (k, candidate) in ItemId::SIML.iter().enumerate() {
                let damage = out.siml_damages(e, k);

                assert_eq!(damage.items.len(), base_len + 2, "{candidate:?}");
                if !candidate.deals_damage() || Some(*candidate) == owned {
                    assert_eq!(&damage.items[base_len..], &[0, 0], "{candidate:?}");
                }
                assert_eq!(damage.runes.len(), runes_len, "{candidate:?}");
            }
        }
    }
}

#[test]
fn backend_decoding_rejects_zero_level_but_accepts_255() {
    let config = types::BINCODE_CONFIG;
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
    // Source grants 6% melee versus 4.8% ranged per stack, .625 AS ratio.
    close(
        mega.current_player.current_stats.attack_speed
            - mini.current_player.current_stats.attack_speed,
        0.045,
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
    use math::const_eval::{ConstEnemy, ConstInput};
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
    const INFERRED: math::const_eval::ConstOutput<
        { ChampionId::Gnar.number_of_abilities() },
        1,
        1,
    > = INPUT.eval();
    const EXPLICIT: math::const_eval::ConstOutput<
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
        bonus_attack_speed_ratio: types::AttackSpeedRatio::new(0.0),
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
        level: NonZeroU8::new(18).unwrap(),
        adaptive_type: math::AdaptiveType::Physical,
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
            GameMap::SummonersRift,
        );
        close(sim.current_stats.max_health, 2050.4); // 2000 + 300 * .15 * 1.12
    }
    let sim = state.simulated_purchase(
        ItemId::Fimbulwinter,
        &[ItemId::WarmogsArmor],
        ChampionId::Gnar,
        Dragons::default(),
        GameMap::SummonersRift,
    );
    close(sim.current_stats.max_health, 2935.2); // 2000 + (550 + 1900 * .15) * 1.12
}

#[test]
fn coup_de_grace_and_cut_down_depend_on_enemy_health() {
    let enemy = |health_ratio: f32| {
        EnemyState {
            current_stats: Some(EnemyStats {
                armor: 50.0,
                current_health: 1000.0 * health_ratio,
                magic_resist: 50.0,
                max_health: 1000.0,
            }),
            ..Default::default()
        }
        .full_state(ResistShred::new(&PlayerStats::default()), false)
    };

    const BOTH: &[RuneId] = &[RuneId::CoupDeGrace, RuneId::CutDown];

    for (health_ratio, coup_de_grace, cut_down) in [
        (0.2, 1.08, 1.0),
        (0.4, 1.0, 1.0),
        (0.5, 1.0, 1.0),
        (0.6, 1.0, 1.0),
        (1.0, 1.0, 1.08),
    ] {
        let state = enemy(health_ratio);

        close(
            state.rune_damage_modifier(&[RuneId::CoupDeGrace]),
            coup_de_grace,
        );
        close(state.rune_damage_modifier(&[RuneId::CutDown]), cut_down);
        close(state.rune_damage_modifier(BOTH), coup_de_grace * cut_down);
    }

    close(enemy(0.2).rune_damage_modifier(&[RuneId::LastStand]), 1.0);
}

#[test]
fn health_dependent_runes_no_longer_buff_every_target() {
    let mut modifiers = Modifiers::default();
    modifiers.apply_loadout(
        &[],
        &[RuneId::CoupDeGrace, RuneId::CutDown],
        &PlayerStats::default(),
    );

    close(modifiers.damages.global_mod, 1.0);
}

#[test]
fn mega_gnar_gains_its_wiki_stat_increases() {
    // Level 1 and level 18 increases from the Mega Gnar template of the wiki
    for (level, health, attack_damage, armor, magic_resist) in
        [(1, 100.0, 6.0, 3.5, 3.5), (18, 831.0, 48.5, 54.5, 63.0)]
    {
        let level = NonZeroU8::new(level).unwrap();
        let mini = BasicStats::base_stats(ChampionId::Gnar, level, false);
        let mega = BasicStats::base_stats(ChampionId::Gnar, level, true);

        close(mega.max_health - mini.max_health, health);
        close(mega.attack_damage - mini.attack_damage, attack_damage);
        close(mega.armor - mini.armor, armor);
        close(mega.magic_resist - mini.magic_resist, magic_resist);
    }
}

#[test]
fn poppy_steadfast_presence_increases_total_resists() {
    let control = inferred_stats(ChampionId::Garen, 11, 0, &[]);
    let poppy = inferred_stats(ChampionId::Poppy, 11, 0, &[]);

    close(poppy.armor, control.armor * 1.16);
    close(poppy.magic_resist, control.magic_resist * 1.16);
}

fn player_stats(items: &[ItemId], item_exceptions: &[ValueException], mega: bool) -> PlayerStats {
    let mut input = game(items, &[], &[]);
    input.active_player.data.item_exceptions = item_exceptions;
    input.active_player.data.is_mega_gnar = mega;
    calculator(input).current_player.current_stats
}

#[test]
fn manamune_awe_scales_with_maximum_mana() {
    let without = player_stats(&[], &[], false);
    let with = player_stats(&[ItemId::Manamune], &[], false);

    // 35 attack damage from its stats, plus 2% of the maximum mana
    close(
        with.attack_damage - without.attack_damage,
        35.0 + 0.02 * with.max_mana,
    );
}

#[test]
fn warmogs_vitality_scales_only_health_from_items() {
    let without = player_stats(&[], &[], false);
    let with = player_stats(&[ItemId::WarmogsArmor], &[], false);

    close(with.max_health - without.max_health, 1000.0 * 1.12);
}

#[test]
fn hubris_eminence_depends_on_the_attack_type() {
    let stacks = [ValueException::pack_item_id(ItemId::Hubris, 5)];

    for (mega, attack_damage) in [(false, 10.0 + 2.0 * 5.0), (true, 15.0 + 3.0 * 5.0)] {
        let without = player_stats(&[], &[], mega);
        let with = player_stats(&[], &stacks, mega);

        close(with.attack_damage - without.attack_damage, attack_damage);
    }
}

#[test]
fn juice_of_power_grants_only_the_adaptive_stat() {
    let without = player_stats(&[], &[], false);
    let with = player_stats(&[ItemId::JuiceOfPower], &[], false);

    // Gnar is physical, so it only gains attack damage
    close(
        with.attack_damage - without.attack_damage,
        18.0 + 0.1 * without.attack_damage,
    );
    close(with.ability_power, without.ability_power);
}

#[test]
fn demon_kings_crown_stacks_are_the_percentage() {
    let stacks = [ValueException::pack_item_id(ItemId::DemonKingsCrown, 25)];
    let without = player_stats(&[], &[], false);
    let with = player_stats(&[], &stacks, false);

    close(with.armor, without.armor * 1.25);
}

#[test]
fn warmogs_vitality_amplifies_health_converted_by_awe() {
    let base = player_stats(&[], &[], false).max_health;
    let fimbulwinter = player_stats(&[ItemId::Fimbulwinter], &[], false).max_health;

    // Warmog's adds 1000 health and amplifies it with the health of Fimbulwinter,
    // including the health Awe converts from mana, whatever the order of the items
    for items in [
        &[ItemId::WarmogsArmor, ItemId::Fimbulwinter][..],
        &[ItemId::Fimbulwinter, ItemId::WarmogsArmor][..],
    ] {
        close(
            player_stats(items, &[], false).max_health - base,
            (fimbulwinter - base + 1000.0) * 1.12,
        );
    }
}

#[test]
fn inferred_warmogs_vitality_amplifies_health_converted_by_awe() {
    let enemy = |items: &[ItemId]| {
        let exceptions = items
            .iter()
            .map(|&item_id| ValueException::pack_item_id(item_id, 0))
            .collect::<Vec<_>>();

        EnemyState {
            champion_id: ChampionId::Garen,
            level: NonZeroU8::new(18).unwrap(),
            items,
            item_exceptions: &exceptions,
            base_stats: SimpleStats {
                armor: 100.0,
                magic_resist: 100.0,
                max_health: 1000.0,
            },
            ..Default::default()
        }
        .full_state(ResistShred::new(&PlayerStats::default()), false)
        .current_stats
        .max_health
    };

    let fimbulwinter = enemy(&[ItemId::Fimbulwinter]);

    for items in [
        &[ItemId::WarmogsArmor, ItemId::Fimbulwinter][..],
        &[ItemId::Fimbulwinter, ItemId::WarmogsArmor][..],
    ] {
        close(
            enemy(items) - 1000.0,
            (fimbulwinter - 1000.0 + 1000.0) * 1.12,
        );
    }
}

#[test]
fn dragonheart_multiplies_health_after_warmogs_vitality() {
    let souls = [ValueException::pack_item_id(ItemId::Dragonheart, 3)];
    let warmogs = player_stats(&[ItemId::WarmogsArmor], &[], false);
    let with_souls = player_stats(&[ItemId::WarmogsArmor], &souls, false);

    // 3 souls increase the total health by 12%, including Warmog's bonus
    close(with_souls.max_health, warmogs.max_health * 1.12);
    close(with_souls.attack_damage, warmogs.attack_damage * 1.12);
}

#[test]
fn inferred_dragonheart_multiplies_health_after_warmogs_vitality() {
    let enemy = |exceptions: &[ValueException]| {
        EnemyState {
            champion_id: ChampionId::Garen,
            level: NonZeroU8::new(18).unwrap(),
            items: &[ItemId::WarmogsArmor],
            item_exceptions: exceptions,
            base_stats: SimpleStats {
                armor: 100.0,
                magic_resist: 100.0,
                max_health: 1000.0,
            },
            ..Default::default()
        }
        .full_state(ResistShred::new(&PlayerStats::default()), false)
        .current_stats
    };

    let warmogs = ValueException::pack_item_id(ItemId::WarmogsArmor, 0);

    // In either order, 25% from Demon King's Crown applies to Warmog's bonus too
    for exceptions in [
        [
            ValueException::pack_item_id(ItemId::DemonKingsCrown, 25),
            warmogs,
        ],
        [
            warmogs,
            ValueException::pack_item_id(ItemId::DemonKingsCrown, 25),
        ],
    ] {
        close(
            enemy(&exceptions).max_health,
            (1000.0 + 1000.0 * 1.12) * 1.25,
        );
    }
}

#[test]
fn aram_changes_apply_to_their_closures_only_on_aram() {
    let ctx = |map| math::Ctx {
        ability_power: 100.0,
        ability_levels: AbilityLevels {
            q: 1,
            w: 1,
            e: 1,
            r: 1,
        },
        map: GameMap::mode(map),
        ..Default::default()
    };

    // 45 base damage (ignoring AD), with 60% AP on Summoner's Rift and 70% in ARAM
    let akali_q = |map| math::libgen::champions::akali::akali_q(&ctx(map));
    close(akali_q(GameMap::SummonersRift), 45.0 + 60.0);
    close(akali_q(GameMap::Unknown), 45.0 + 60.0);
    close(akali_q(GameMap::Aram), 45.0 + 70.0);
    close(akali_q(GameMap::AramMayhem), 45.0 + 70.0);

    // Both the base damage and the ratio of Kassadin's E change
    let kassadin_e = |map| math::libgen::champions::kassadin::kassadin_e(&ctx(map));
    close(kassadin_e(GameMap::SummonersRift), 70.0 + 70.0);
    close(kassadin_e(GameMap::Aram), 80.0 + 75.0);
}

#[test]
fn aram_multiplies_the_total_attack_speed_of_some_champions() {
    let attack_speed = |champion_id, game_map| {
        let mut input = game(&[], &[], &[]);
        input.active_player.data.champion_id = champion_id;
        input.game_map = game_map;
        calculator(input).current_player.current_stats.attack_speed
    };

    let rift = attack_speed(ChampionId::Xayah, GameMap::SummonersRift);
    close(attack_speed(ChampionId::Xayah, GameMap::Aram), rift * 1.03);
    close(
        attack_speed(ChampionId::Xayah, GameMap::AramMayhem),
        rift * 1.03,
    );

    // Champions without the change keep it
    let akali = attack_speed(ChampionId::Akali, GameMap::SummonersRift);
    close(attack_speed(ChampionId::Akali, GameMap::Aram), akali);
}

fn player_stats_on(
    items: &[ItemId],
    item_exceptions: &[ValueException],
    map: GameMap,
) -> PlayerStats {
    let mut input = game(items, &[], &[]);
    input.active_player.data.item_exceptions = item_exceptions;
    input.game_map = map;
    calculator(input).current_player.current_stats
}

#[test]
fn arena_changes_base_stats_from_champion_data_and_its_changes() {
    let level = NonZeroU8::new(18).unwrap();
    let health = |champion: ChampionId, map| champion.map_base_stats(level, false, map).max_health;

    // Akali grows 2 more health per level in Arena, over 17 levels
    close(
        health(ChampionId::Akali, GameMap::Arena)
            - health(ChampionId::Akali, GameMap::SummonersRift),
        2.0 * 17.0,
    );
    // Azir passively gains {{pp|0 to 187}} base health
    close(
        health(ChampionId::Azir, GameMap::Arena) - health(ChampionId::Azir, GameMap::SummonersRift),
        187.0,
    );
    // ARAM keeps them
    close(
        health(ChampionId::Akali, GameMap::Aram),
        health(ChampionId::Akali, GameMap::SummonersRift),
    );
}

#[test]
fn arena_base_stat_bonuses_keep_their_parsed_level_progression() {
    for level in 1..=18 {
        let level = NonZeroU8::new(level).unwrap();
        let health = |map| {
            ChampionId::Azir
                .map_base_stats(level, false, map)
                .max_health
        };
        close(
            health(GameMap::Arena) - health(GameMap::SummonersRift),
            11.0 * (level.get() - 1) as f32,
        );
    }
}

#[test]
fn map_attack_speed_purchase_matches_inferred_build() {
    for map in [GameMap::Aram, GameMap::AramMayhem, GameMap::SummonersRift] {
        let infer = |items| {
            let mut input = game(items, &[], &[]);
            input.active_player.data.champion_id = ChampionId::Xayah;
            input.game_map = map;
            calculator(input)
        };
        let bare = infer(&[]);
        let player = &bare.current_player;
        let state = SelfState {
            bonus_attack_speed_ratio: ChampionId::Xayah
                .stats_on(map)
                .bonus_attack_speed_ratio(player.current_stats.attack_speed),
            current_stats: player.current_stats,
            base_stats: player.base_stats,
            adaptive_type: player.adaptive_type,
            ability_levels: game(&[], &[], &[]).active_player.abilities,
            level: player.level,
            stacks: 0.0,
        };
        let purchased = state.simulated_purchase(
            ItemId::NashorsTooth,
            &[],
            ChampionId::Xayah,
            Dragons::default(),
            map,
        );
        let inferred = infer(&[ItemId::NashorsTooth]);
        close(
            purchased.current_stats.attack_speed,
            inferred.current_player.current_stats.attack_speed,
        );
        close(
            purchased.bonus_attack_speed_ratio.get(),
            inferred
                .monster_rows()
                .next()
                .unwrap()
                .ctx
                .bonus_attack_speed_ratio
                .get(),
        );
    }
}

#[test]
fn arena_changes_item_stats() {
    let ad = |map| {
        player_stats_on(&[ItemId::BladeOfTheRuinedKing], &[], map).attack_damage
            - player_stats_on(&[], &[], map).attack_damage
    };

    // 40 attack damage, and 30 in Arena
    close(ad(GameMap::SummonersRift), 40.0);
    close(ad(GameMap::Arena), 30.0);
    close(ad(GameMap::Aram), 40.0);
}

#[test]
fn aram_changes_hubris_eminence_through_its_constants() {
    let stacks = [ValueException::pack_item_id(ItemId::Hubris, 5)];
    let eminence = |map| {
        player_stats_on(&[], &stacks, map).attack_damage
            - player_stats_on(&[], &[], map).attack_damage
    };

    // Gnar is ranged: 10 + 2 per stack, and 10 + 1 per stack in ARAM
    close(eminence(GameMap::SummonersRift), 10.0 + 2.0 * 5.0);
    close(eminence(GameMap::Aram), 10.0 + 1.0 * 5.0);
    close(eminence(GameMap::AramMayhem), 10.0 + 1.0 * 5.0);
}

#[test]
fn arena_changes_retribution_and_plating() {
    let retribution = |missing, map| RiotFormulas::get_bloodmail_retribution(missing, map);

    // 1% per 7% missing health up to 10%, and 1.25% per 5% up to 17.5% in Arena
    close(retribution(0.35, GameMap::SummonersRift), 0.05);
    close(retribution(0.7, GameMap::SummonersRift), 0.1);
    close(retribution(0.35, GameMap::Arena), 0.0875);
    close(retribution(0.7, GameMap::Arena), 0.175);

    let plating = |map: GameMap| {
        math::Ctx {
            flags: math::CtxFlags::target(true, false, false, false),
            map: map.mode(),
            ..Default::default()
        }
        .steelcaps_effect()
    };

    close(plating(GameMap::SummonersRift), 0.9);
    close(plating(GameMap::Arena), 0.86);
}

#[test]
fn aram_changes_rune_closures() {
    let ctx = |map: GameMap| math::Ctx {
        ability_power: 100.0,
        level: 1.0,
        map: map.mode(),
        ..Default::default()
    };

    // 70 at level 1, with 5% AP, and 10% AP in ARAM
    let electrocute = |map| math::libgen::runes::electrocute::electrocute(&ctx(map));
    close(electrocute(GameMap::SummonersRift), 70.0 + 5.0);
    close(electrocute(GameMap::Aram), 70.0 + 10.0);
}

#[test]
fn item_critical_damage_adds_to_the_champion_base_unscaled() {
    // A real game: Ashe's critical damage is 100 (200 * 0.5) and 130 with Infinity Edge
    let enemies = [InputMinData {
        champion_id: ChampionId::Aatrox,
        level: NonZeroU8::new(18).unwrap(),
        ..Default::default()
    }];
    let mut input = game(&[ItemId::InfinityEdge], &[], &enemies);
    input.active_player.data.champion_id = ChampionId::Ashe;
    let output = calculator(input);

    close(output.current_player.current_stats.crit_damage, 130.0);
}

#[test]
fn simulated_critical_damage_adds_unscaled() {
    let state = SelfState {
        bonus_attack_speed_ratio: types::AttackSpeedRatio::new(0.0),
        stacks: 0.0,
        ability_levels: AbilityLevels::default(),
        current_stats: PlayerStats {
            crit_damage: 100.0,
            ..PlayerStats::default()
        },
        base_stats: BasicStats::default(),
        level: NonZeroU8::MIN,
        adaptive_type: math::AdaptiveType::Physical,
    };
    let purchase = state.simulated_purchase(
        ItemId::InfinityEdge,
        &[],
        ChampionId::Ashe,
        Dragons::default(),
        GameMap::SummonersRift,
    );

    close(purchase.current_stats.crit_damage, 130.0);
}

#[test]
fn bonus_attack_speed_uses_champion_ratio_and_includes_level_growth() {
    for champion in [ChampionId::Caitlyn, ChampionId::Gnar] {
        for level in [1, 18] {
            let mut input = game(&[], &[], &[]);
            input.active_player.data.champion_id = champion;
            input.active_player.data.level = NonZeroU8::new(level).unwrap();
            let out = calculator(input);
            let ctx = out.monster_rows().next().unwrap().ctx;
            let wiki = champion.stats_on(GameMap::Unknown);
            let growth = wiki.attack_speed.per_level * if level == 1 { 0.0 } else { 17.0 };
            close(ctx.bonus_attack_speed_ratio.get(), growth / 100.0);
            close(
                ctx.attack_speed,
                wiki.attack_speed.base + wiki.attack_speed_ratio * growth / 100.0,
            );
        }
    }
}

#[test]
fn live_and_simulated_bonus_attack_speed_use_flat_base_and_ratio() {
    let mut input = live_game();
    input.all_players[0].champion_name = "Caitlyn";
    let wiki = ChampionId::Caitlyn.stats_on(GameMap::SummonersRift);
    assert!((wiki.attack_speed.base - wiki.attack_speed_ratio).abs() > 0.01);
    let observed = wiki.attack_speed.base + 1.25 * wiki.attack_speed_ratio;
    input.active_player.champion_stats.attack_speed = observed;
    let out = realtime(&input).unwrap();
    let original = out.enemy_damages(0).ctx;
    close(original.bonus_attack_speed_ratio.get(), 1.25);
    assert!((original.base_attack_speed - wiki.attack_speed.base).abs() > 0.01);
    let purchased = out.siml_damages(0, sim_index(ItemId::NashorsTooth)).ctx;
    close(purchased.bonus_attack_speed_ratio.get(), 1.75);
    close(
        purchased.attack_speed,
        observed + 0.5 * wiki.attack_speed_ratio,
    );
}

#[test]
fn invalid_attack_speed_ratio_inputs_have_no_bonus() {
    let original = *ChampionId::Caitlyn.stats_on(GameMap::Unknown);
    for ratio in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        let wiki = math::libgen::WikiStats {
            attack_speed_ratio: ratio,
            ..original
        };
        close(wiki.bonus_attack_speed_ratio(1.0).get(), 0.0);
    }
    for observed in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        close(original.bonus_attack_speed_ratio(observed).get(), 0.0);
    }
    close(
        original
            .bonus_attack_speed_ratio(
                original.attack_speed.base - 0.25 * original.attack_speed_ratio,
            )
            .get(),
        -0.25,
    );
}

#[test]
fn inferred_lethal_tempo_stacks_feed_the_damage_amplification_on_each_map() {
    let exceptions = [ValueException::pack_rune_id(RuneId::LethalTempo, 6)];
    for (champion, mega, attack, normal, arena, damage) in [
        (ChampionId::Gnar, true, AttackType::Melee, 0.36, 1.08, 30.0),
        (
            ChampionId::Caitlyn,
            false,
            AttackType::Ranged,
            0.288,
            0.72,
            20.01,
        ),
    ] {
        for (map, extra) in [(GameMap::SummonersRift, normal), (GameMap::Arena, arena)] {
            let mut input = game(&[], &[RuneId::LethalTempo], &[]);
            input.active_player.data.champion_id = champion;
            input.active_player.data.is_mega_gnar = mega;
            input.game_map = map;
            let baseline = calculator(input).monster_rows().next().unwrap().ctx;
            input.active_player.rune_exceptions = &exceptions;
            let out = calculator(input);
            let ctx = out.monster_rows().next().unwrap().ctx;
            close(
                ctx.bonus_attack_speed_ratio.get() - baseline.bonus_attack_speed_ratio.get(),
                extra,
            );
            let actual = RuneId::LethalTempo.eval(&ctx, attack)[0];
            let expected = damage * (1.0 + ctx.bonus_attack_speed_ratio.get());
            // Rounded wiki samples may become an inferred growth expression.
            assert!(
                (actual - expected).abs() <= expected.abs() * 0.001,
                "{actual} != {expected}"
            );
        }
    }
}
