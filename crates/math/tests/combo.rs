#![cfg(feature = "engine")]

use core::num::NonZeroU8;
use math::{
    AbilityId, AbilityName, AttackRole, ChampionId, ItemId,
    combo::{ComboError, MAX_COMBO_STEPS},
    model::*,
    realtime::{RealtimeError, realtime, realtime_with_combo},
    riot::*,
};

fn game(champion: &'static str, items: &[ItemId]) -> RiotRealtime<'static> {
    let stats = PlayerStats {
        attack_damage: 200.0,
        ability_power: 100.0,
        max_health: 2000.0,
        current_health: 2000.0,
        attack_speed: 1.0,
        crit_damage: 175.0,
        armor_penetration_percent: 1.0,
        magic_penetration_percent: 1.0,
        ..PlayerStats::default()
    };

    let player = |name, id, team, items: Box<[RiotItems]>| RiotAllPlayers {
        champion_name: name,
        raw_champion_name: "",
        raw_skin_name: "",
        riot_id: id,
        team,
        position: "TOP",
        level: NonZeroU8::new(18).unwrap(),
        items,
        scores: RiotScoreboard {
            kills: 0,
            deaths: 0,
            assists: 0,
            creep_score: 0,
        },
    };

    let items = items
        .iter()
        .map(|item| RiotItems {
            item_id: item.riot_id(),
        })
        .collect();

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
            player(champion, "self", "ORDER", items),
            player("Aatrox", "enemy", "CHAOS", Box::new([])),
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

fn combo_of<'a>(out: &'a Realtime) -> ComboRow<'a> {
    out.damages.combo(0).expect("the table has combo columns")
}

#[test]
fn no_combo_adds_no_columns() {
    let input = game("Gnar", &[]);
    let out = realtime(&input).unwrap();

    assert!(out.combo.is_empty());
    assert_eq!(out.damages.layout().combo, 0);
    assert!(out.damages.combo(0).is_none());
}

#[test]
fn listed_procs_imply_one_attack_per_group() {
    let input = game("Gnar", &[ItemId::LichBane]);
    let q = ChampionId::Gnar
        .metadata()
        .iter()
        .find(|metadata| metadata.attributes.role() == AttackRole::Standalone)
        .map(|metadata| ComboStep::Ability(metadata.kind))
        .unwrap();
    let lich = ComboStep::Item(ItemId::LichBane);
    let out = realtime_with_combo(
        &input,
        &[q, lich, ComboStep::OnhitMin, ComboStep::OnhitMin, q],
    )
    .unwrap();

    assert_eq!(
        &*out.combo,
        &[
            q,
            ComboStep::Attack,
            ComboStep::Attack,
            lich,
            ComboStep::OnhitMin,
            ComboStep::OnhitMin,
            q
        ]
    );
}

#[test]
fn attacks_subtract_health_and_report_the_kill() {
    let input = game("Gnar", &[]);
    let combo = [ComboStep::Attack; 40];
    let out = realtime_with_combo(&input, &combo).unwrap();
    let row = combo_of(&out);
    let start = out.enemies[0].current_stats.current_health as i32;
    let attack = out.enemy_damages(0).attacks.basic_attack;

    assert_eq!(row.steps.len(), 40);
    assert!(row.steps.iter().all(|&damage| (damage - attack).abs() <= 1));
    assert!((row.total - row.steps.iter().sum::<i32>()).abs() <= 40);
    assert!((row.final_health - (start - row.total)).abs() <= 1);

    let kill = row.kill_step.expect("40 attacks kill the target");
    let before: i32 = row.steps[..kill].iter().sum();
    let through: i32 = row.steps[..=kill].iter().sum();

    assert!(before < start && through + 1 >= start);
}

#[test]
fn every_hit_items_apply_on_each_attack() {
    let plain = game("Gnar", &[]);
    let nashor = game("Gnar", &[ItemId::NashorsTooth]);
    let combo = [ComboStep::Attack];

    let plain = realtime_with_combo(&plain, &combo).unwrap();
    let nashor = realtime_with_combo(&nashor, &combo).unwrap();
    let item = nashor.enemy_damages(0).items[0];

    assert!(item > 0);
    assert!((combo_of(&nashor).steps[0] - combo_of(&plain).steps[0] - item).abs() <= 1);
}

#[test]
fn ruined_king_deals_less_as_the_target_loses_health() {
    let input = game("Gnar", &[ItemId::BladeOfTheRuinedKing]);
    let out = realtime_with_combo(&input, &[ComboStep::Attack; 10]).unwrap();
    let row = combo_of(&out);

    assert!(row.steps.windows(2).all(|pair| pair[1] <= pair[0]));
    assert!(row.steps[9] < row.steps[0]);
}

#[test]
fn hecarim_rampage_stacks_grow_per_cast() {
    let input = game("Hecarim", &[]);
    let q = ComboStep::Ability(AbilityId::Q(AbilityName::Void));
    let out = realtime_with_combo(&input, &[q, q, q, q]).unwrap();
    let steps = combo_of(&out).steps;

    // The table evaluates Q with no stacks, as the first cast
    assert!((steps[0] - out.enemy_damages(0).abilities[0]).abs() <= 1);
    assert!(steps[1] > steps[0] && steps[2] > steps[1]);
    // Two stacks at most
    assert!((steps[3] - steps[2]).abs() <= 1);
}

#[test]
fn black_cleaver_raises_later_physical_damage() {
    let input = game("Gnar", &[ItemId::BlackCleaver]);
    let out = realtime_with_combo(&input, &[ComboStep::Crit; 7]).unwrap();
    let steps = combo_of(&out).steps;

    // Five stacks of Carve, then no more
    assert!(steps[5] > steps[0]);
    assert!((steps[6] - steps[5]).abs() <= 1);
}

#[test]
fn invalid_combos_are_rejected() {
    let input = game("Gnar", &[ItemId::NashorsTooth]);
    let error = |combo: &[ComboStep]| match realtime_with_combo(&input, combo) {
        Err(RealtimeError::Combo(error)) => error,
        other => panic!("expected a combo error, got {other:?}"),
    };

    let lich = ComboStep::Item(ItemId::LichBane);
    let nashor = ComboStep::Item(ItemId::NashorsTooth);
    let unknown = AbilityId::Q(AbilityName::_8);

    assert_eq!(error(&[lich]), ComboError::NotOwned(lich));
    assert_eq!(error(&[nashor]), ComboError::EveryHitListed(nashor));
    assert_eq!(
        error(&[ComboStep::Ability(unknown)]),
        ComboError::UnknownAbility(unknown)
    );
    assert_eq!(
        error(&[ComboStep::Attack; MAX_COMBO_STEPS + 1]),
        ComboError::TooLong(MAX_COMBO_STEPS + 1)
    );
}

#[test]
fn candidate_rows_have_combo_columns() {
    let input = game("Gnar", &[]);
    let out = realtime_with_combo(&input, &[ComboStep::Attack]).unwrap();
    let nashor = ItemId::SIML
        .iter()
        .position(|&item| item == ItemId::NashorsTooth)
        .unwrap();

    let base = combo_of(&out).total;
    let with_nashor = out.damages.combo(1 + nashor).unwrap().total;

    assert!(with_nashor > base);
}
