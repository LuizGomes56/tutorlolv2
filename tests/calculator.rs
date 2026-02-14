use serde_json::json;
use tutorlolv2::{helpers::*, model::*, *};
use tutorlolv2_gen::bitset_items;

#[test]
pub fn test_calculator() {
    const CHAMPION_ID: ChampionId = ChampionId::Neeko;
    const LEVEL: u8 = 18;

    let items = [ItemId::BladeOfTheRuinedKing, ItemId::NashorsTooth];
    let items_meta = get_items_data(&bitset_items(items), AttackType::Melee);

    const CURRENT_STATS: Stats<f32> = Stats {
        ability_power: 100.0,
        armor: 100.0,
        armor_penetration_flat: 0.0,
        armor_penetration_percent: 0.0,
        attack_damage: 100.0,
        attack_range: 100.0,
        attack_speed: 100.0,
        crit_chance: 100.0,
        crit_damage: 100.0,
        current_health: 100.0,
        magic_penetration_flat: 0.0,
        magic_penetration_percent: 0.0,
        magic_resist: 100.0,
        max_health: 100.0,
        max_mana: 100.0,
        current_mana: 100.0,
    };

    const BASE_STATS: BasicStats<f32> = base_stats_bf32(CHAMPION_ID, LEVEL, false);
    const BONUS_STATS: BasicStats<f32> = bonus_stats!(BasicStats::<f32>(CURRENT_STATS, BASE_STATS) {
        armor,
        max_health,
        attack_damage,
        magic_resist,
        max_mana
    });

    const ADAPTATIVE_TYPE: AdaptativeType = {
        let this =
            RiotFormulas::adaptative_type(BONUS_STATS.attack_damage, CURRENT_STATS.ability_power);
        let default = CHAMPION_ID.cache().adaptative_type;
        match this {
            Some(x) => x,
            None => default,
        }
    };

    let self_state = SelfState {
        stacks: 0.0,
        ability_levels: AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        },
        current_stats: CURRENT_STATS,
        bonus_stats: BONUS_STATS,
        base_stats: BASE_STATS,
        level: LEVEL,
        adaptative_type: ADAPTATIVE_TYPE,
    };

    const SHRED: ResistShred = ResistShred::new(&CURRENT_STATS);

    const E_CHAMPION_ID: ChampionId = ChampionId::Aatrox;

    const ENEMY_STATE: EnemyState<'_> = EnemyState {
        current_stats: Some(EnemyStats {
            armor: 599251.0,
            current_health: 599251.0,
            magic_resist: 599251.0,
            max_health: 599251.0,
            missing_health: 599251.0,
        }),
        base_stats: base_stats_sf32(E_CHAMPION_ID, LEVEL, false),
        items: &[],
        stacks: 0,
        champion_id: E_CHAMPION_ID,
        earth_dragons: 0,
        level: LEVEL,
        item_exceptions: &[],
    };
    const FULL_STATE: EnemyFullState = get_enemy_state(ENEMY_STATE, SHRED, false);

    let ctx = get_eval_ctx(&self_state, &FULL_STATE);
    let damages = item_id_eval_damage(
        &ctx,
        &mut Default::default(),
        &items_meta.metadata,
        &items_meta.closures,
        Modifiers::new(&ctx),
    );

    std::fs::write(
        "test_calc.txt",
        serde_json::to_string_pretty(&json!({
            "enemy_state": ENEMY_STATE,
            "full_state": FULL_STATE,
            "ctx": ctx,
            "damages": damages,
            "metadata": items_meta.metadata,
            "closures": format!("{:?}", items_meta.closures)
        }))
        .unwrap(),
    )
    .unwrap()
}
