use serde_json::json;
use tutorlolv2::{helpers::*, model::*, *};
use tutorlolv2_gen::bitset_items;

#[test]
pub fn test_calculator() {
    let champion_id = ChampionId::Neeko;
    let level = 18;

    let items = [ItemId::BladeOfTheRuinedKing, ItemId::NashorsTooth];
    let items_meta = get_items_data(&bitset_items(items), AttackType::Melee);

    let mut current_stats = unsafe { core::mem::transmute::<_, Stats<f32>>([100f32; 16]) };
    current_stats.armor_penetration_flat = 0.0;
    current_stats.armor_penetration_percent = 0.0;
    current_stats.magic_penetration_flat = 0.0;
    current_stats.magic_penetration_percent = 0.0;

    let base_stats = base_stats_bf32(champion_id, level, false);
    let bonus_stats = bonus_stats!(BasicStats::<f32>(current_stats, base_stats) {
        armor,
        health,
        attack_damage,
        magic_resist,
        mana
    });

    let adaptative_type =
        RiotFormulas::adaptative_type(bonus_stats.attack_damage, current_stats.ability_power)
            .unwrap_or(champion_id.cache().adaptative_type);

    let self_state = SelfState {
        stacks: 0.0,
        ability_levels: AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        },
        current_stats,
        bonus_stats,
        base_stats,
        level,
        adaptative_type,
    };

    let shred = ResistShred::new(&current_stats);

    let e_champion_id = ChampionId::Aatrox;

    let enemy_state = EnemyState {
        base_stats: base_stats_sf32(e_champion_id, level, false),
        items: &[],
        stacks: 0,
        champion_id: e_champion_id,
        earth_dragons: 0,
        level,
        item_exceptions: &[],
    };
    let full_state = get_enemy_state(enemy_state, shred, false);

    let ctx = get_eval_ctx(&self_state, &full_state);
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
            "ctx": ctx,
            "damages": damages,
            "metadata": items_meta.metadata,
            "closures": format!("{:?}", items_meta.closures)
        }))
        .unwrap(),
    )
    .unwrap()
}
