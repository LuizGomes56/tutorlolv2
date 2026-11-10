#![cfg(feature = "engine")]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use tutorlolv2::{calculator::*, model::*, *};

macro_rules! compare_calculators {
    ($name:ident, $game:expr, $enemies:expr) => {
        #[test]
        fn $name() {
            const GAME: InputGame<'static> = $game;
            const LAYOUT: DamageLayout = calculator_layout(GAME);
            const ARRAY: ArrayOutputGame<LAYOUT, $enemies> =
                calculator_array::<LAYOUT, $enemies>(GAME);
            // Calling the array entry point at runtime must also produce the same result.
            let runtime_array = calculator_array::<LAYOUT, $enemies>(GAME);
            let boxed = calculator(GAME);
            assert_eq!(ARRAY.current_player, boxed.current_player);
            assert_eq!(ARRAY.enemies.as_slice(), boxed.enemies.as_ref());
            assert_eq!(ARRAY.items_meta.as_slice(), boxed.items_meta.as_ref());
            assert_eq!(ARRAY.runes_meta.as_slice(), boxed.runes_meta.as_ref());
            assert_eq!(ARRAY.tower_damages, boxed.tower_damages);
            assert_eq!(ARRAY.damages.layout(), boxed.damages.layout());
            assert_eq!(ARRAY.damages.rows(), boxed.damages.rows());
            for row in 0..boxed.damages.rows() {
                assert_eq!(ARRAY.damages.row(row), boxed.damages.row(row), "row {row}");
                assert_eq!(
                    runtime_array.damages.row(row),
                    boxed.damages.row(row),
                    "runtime row {row}"
                );
            }
        }
    };
}

compare_calculators!(neeko_with_items_runes_and_enemy, test::GAME, 1);

#[test]
fn macro_infers_dimensions_and_evaluates_at_compile_time() {
    const GAME: InputGame<'static> = test::GAME;
    let output = tutorlolv2::calculator_array!(GAME);
    assert_eq!(output.enemy_damages(0), test::OUT.enemy_damages(0));
    assert_eq!(output.monster_damages(0), test::OUT.monster_damages(0));
}
compare_calculators!(
    monsters_only_and_empty_loadout,
    InputGame {
        active_player: InputActivePlayer {
            runes: &[],
            data: InputMinData {
                items: &[],
                ..test::GAME.active_player.data
            },
            ..test::GAME.active_player
        },
        enemy_players: &[],
        ..test::GAME
    },
    0
);
compare_calculators!(
    mega_gnar_deduplicates_and_filters_loadout,
    InputGame {
        active_player: InputActivePlayer {
            data: InputMinData {
                champion_id: ChampionId::Gnar,
                is_mega_gnar: true,
                items: &[
                    ItemId::BladeOfTheRuinedKing,
                    ItemId::BladeOfTheRuinedKing,
                    ItemId::SorcerersShoes
                ],
                ..test::GAME.active_player.data
            },
            runes: &[
                RuneId::Electrocute,
                RuneId::Electrocute,
                RuneId::AxiomArcanist
            ],
            ..test::GAME.active_player
        },
        ..test::GAME
    },
    1
);
compare_calculators!(
    multiple_enemies_with_map_and_health_effects,
    InputGame {
        active_player: InputActivePlayer {
            current_health: 750.0,
            data: InputMinData {
                champion_id: ChampionId::Ahri,
                items: &[
                    ItemId::DemonicEmbrace,
                    ItemId::NashorsTooth,
                    ItemId::Shadowflame
                ],
                ..test::GAME.active_player.data
            },
            ..test::GAME.active_player
        },
        enemy_players: &[
            test::GAME.enemy_players[0],
            InputMinData {
                champion_id: ChampionId::Ornn,
                items: &[ItemId::PlatedSteelcaps, ItemId::RanduinsOmen],
                stats: Some(EnemyStats {
                    armor: 150.0,
                    magic_resist: 80.0,
                    current_health: 1600.0,
                    max_health: 3000.0
                }),
                ..test::GAME.enemy_players[0]
            },
        ],
        game_map: GameMap::AramMayhem,
        ..test::GAME
    },
    2
);

#[test]
fn generated_formula_tables_match_the_original_pointer_tables() {
    use strum::EnumCount;
    // Each context is shared across the crates through tutorlolv2_types.
    let contexts = [
        test::CTX,
        Ctx {
            attack_damage: 400.0,
            ability_power: 600.0,
            enemy_current_health: 250.0,
            ..test::CTX
        },
    ];
    for ctx in &contexts {
        for i in 0..ChampionId::COUNT {
            let actual = ChampionId::from_repr(i as _).unwrap().data();
            let expected = tutorlolv2_stable::ChampionId::from_repr(i as _)
                .unwrap()
                .data();
            assert_eq!(actual.closures.len(), expected.closures.len());
            for (a, b) in actual.closures.iter().zip(expected.closures) {
                assert_eq!(a(ctx).to_bits(), b(ctx).to_bits(), "champion {i}");
            }
        }
        for i in 0..ItemId::COUNT {
            let actual = ItemId::from_repr(i as _).unwrap().data();
            let expected = tutorlolv2_stable::ItemId::from_repr(i as _).unwrap().data();
            for (a, b) in actual
                .melee
                .iter()
                .chain(&actual.ranged)
                .zip(expected.melee.iter().chain(&expected.ranged))
            {
                assert_eq!(a(ctx).to_bits(), b(ctx).to_bits(), "item {i}");
            }
        }
        for i in 0..RuneId::COUNT {
            let actual = RuneId::from_repr(i as _).unwrap().data();
            let expected = tutorlolv2_stable::RuneId::from_repr(i as _).unwrap().data();
            for (a, b) in actual
                .melee
                .iter()
                .chain(&actual.ranged)
                .zip(expected.melee.iter().chain(&expected.ranged))
            {
                assert_eq!(a(ctx).to_bits(), b(ctx).to_bits(), "rune {i}");
            }
        }
    }
}

#[test]
fn boxed_calculator_matches_original_calculator_for_every_champion() {
    use strum::EnumCount;
    for i in 0..ChampionId::COUNT {
        let mut game = test::GAME;
        game.active_player.data.champion_id = ChampionId::from_repr(i as _).unwrap();
        let actual = calculator(game);
        let original_game = tutorlolv2_stable::model::InputGame {
            active_player: tutorlolv2_stable::model::InputActivePlayer {
                runes: &[tutorlolv2_stable::RuneId::Electrocute],
                rune_exceptions: &[],
                abilities: game.active_player.abilities,
                current_health: 0.0,
                data: tutorlolv2_stable::model::InputMinData {
                    champion_id: tutorlolv2_stable::ChampionId::from_repr(i as _).unwrap(),
                    items: &[
                        tutorlolv2_stable::ItemId::NashorsTooth,
                        tutorlolv2_stable::ItemId::RabadonsDeathcap,
                        tutorlolv2_stable::ItemId::SorcerersShoes,
                    ],
                    item_exceptions: &[],
                    stats: None,
                    stacks: 0,
                    level: game.active_player.data.level,
                    is_mega_gnar: false,
                },
            },
            enemy_players: &[tutorlolv2_stable::model::InputMinData {
                champion_id: tutorlolv2_stable::ChampionId::Aatrox,
                items: &[tutorlolv2_stable::ItemId::ForceOfNature],
                item_exceptions: &[],
                stats: None,
                stacks: 0,
                level: game.active_player.data.level,
                is_mega_gnar: false,
            }],
            dragons: tutorlolv2_stable::model::Dragons::default(),
            game_map: game.game_map,
        };
        let original = tutorlolv2_stable::calculator::calculator(original_game);
        assert_eq!(actual.tower_damages, original.tower_damages, "champion {i}");
        for row in 0..actual.damages.rows() {
            let a = actual.damages.row(row);
            let b = original.damages.row(row);
            assert_eq!(a.ctx, b.ctx, "champion {i}, row {row}");
            assert_eq!(a.abilities, b.abilities, "champion {i}, row {row}");
            assert_eq!(a.items, b.items, "champion {i}, row {row}");
            assert_eq!(a.runes, b.runes, "champion {i}, row {row}");
            assert_eq!(
                (
                    a.attacks.basic_attack,
                    a.attacks.critical_strike,
                    a.attacks.onhit_damage.minimum_damage,
                    a.attacks.onhit_damage.maximum_damage
                ),
                (
                    b.attacks.basic_attack,
                    b.attacks.critical_strike,
                    b.attacks.onhit_damage.minimum_damage,
                    b.attacks.onhit_damage.maximum_damage
                )
            );
        }
    }
}

#[test]
#[should_panic(expected = "calculator layout does not match input")]
fn wrong_layout_is_rejected() {
    const WRONG: DamageLayout = DamageLayout {
        abilities: 0,
        items: 0,
        runes: 0,
    };
    let _ = calculator_array::<WRONG, 1>(test::GAME);
}

#[test]
#[should_panic(expected = "enemy count does not match input")]
fn wrong_enemy_count_is_rejected() {
    let _ = calculator_array::<{ test::LAYOUT }, 0>(test::GAME);
}
