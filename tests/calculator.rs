use serde_json::json;
use tutorlolv2::{
    calculator::{InferStats, get_item_bonus_stats, infer_champion_stats},
    helpers::*,
    model::*,
    *,
};
use tutorlolv2_gen::bitset_items;

#[test]
pub fn test_calculator() {
    const CHAMPION_ID: ChampionId = ChampionId::Neeko;
    const LEVEL: u8 = 18;

    const ITEMS: [ItemId; 3] = [
        ItemId::BladeOfTheRuinedKing,
        ItemId::NashorsTooth,
        ItemId::LordDominiksRegards,
    ];
    let items_meta = get_items_data(&bitset_items(ITEMS), AttackType::Melee);
    let mut modifiers = Modifiers::default();

    let mut stats: Stats<f32> = Stats::default();
    get_item_bonus_stats(&mut stats, &ITEMS, &mut modifiers);

    const INFER: bool = true;

    const ABILITY_LEVELS: AbilityLevels = AbilityLevels {
        q: 5,
        w: 5,
        e: 5,
        r: 3,
    };
    const CURRENT_STATS: Stats<f32> = match INFER {
        true => infer_champion_stats(InferStats {
            item_exceptions: &[],
            rune_exceptions: &[],
            items: &ITEMS,
            runes: &[],
            modifiers: &mut Modifiers::default(),
            dragons: Dragons::default(),
            ability_levels: ABILITY_LEVELS,
            stacks: 0,
            level: LEVEL,
            champion_id: CHAMPION_ID,
            is_mega_gnar: false,
        }),
        false => Stats {
            ability_power: 400.0,
            armor: 120.0,
            armor_penetration_flat: 0.0,
            armor_penetration_percent: 0.0,
            attack_damage: 110.0,
            attack_range: 350.0,
            attack_speed: 1.0,
            crit_chance: 20.0,
            crit_damage: 200.0,
            current_health: 1743.0,
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: 65.0,
            max_health: 2689.0,
            max_mana: 960.0,
            current_mana: 127.0,
        },
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

    const SELF_STATE: SelfState = SelfState {
        stacks: 0.0,
        ability_levels: ABILITY_LEVELS,
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
            armor: 214.0,
            current_health: 4124.0,
            magic_resist: 125.0,
            max_health: 5215.0,
            missing_health: 1.0,
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

    const CTX: Ctx = get_eval_ctx(&SELF_STATE, &FULL_STATE);
    let damages = item_id_eval_damage(
        &CTX,
        &mut Default::default(),
        &items_meta.metadata,
        &items_meta.closures,
        Modifiers::new(&CTX),
    );

    std::fs::write(
        "test_calc.txt",
        serde_json::to_string_pretty(&json!({
            "stats": stats,
            "enemy_state": ENEMY_STATE,
            "full_state": FULL_STATE,
            "ctx": CTX,
            "damages": damages,
            "metadata": items_meta.metadata,
            "closures": format!("{:?}", items_meta.closures)
        }))
        .unwrap(),
    )
    .unwrap()
}
