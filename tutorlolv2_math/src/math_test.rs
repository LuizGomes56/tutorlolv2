use crate::{
    AbilityLevels, BasicStats, ConstDamages, EnemyFullState, EnemyState, Modifiers, ResistShred,
    ResistValue, RiotFormulas, SelfState, SimpleStats, Stats, bonus_stats, const_get_damages,
    get_enemy_state, get_eval_ctx,
};
use tutorlolv2_gen::{AdaptativeType, AttackType, ChampionId, EvalContext};

const _CHAMPION_ID: ChampionId = ChampionId::Neeko;
const _LEVEL: u8 = 18;
const _BASE_STATS: BasicStats<f32> = BasicStats::base_stats(_CHAMPION_ID, _LEVEL, false);
const _CURRENT_STATS: Stats<f32> = Stats {
    ability_power: 1000.0,
    armor: _BASE_STATS.armor,
    magic_penetration_flat: 10.0,
    magic_penetration_percent: 0.0,
    armor_penetration_flat: 0.0,
    armor_penetration_percent: 0.0,
    attack_damage: _BASE_STATS.attack_damage,
    magic_resist: _BASE_STATS.magic_resist,
    attack_range: 550.0,
    attack_speed: 1.0,
    crit_chance: 0.0,
    crit_damage: 175.0,
    current_health: _BASE_STATS.health,
    health: _BASE_STATS.health + 1000.0,
    mana: _BASE_STATS.mana,
    current_mana: _BASE_STATS.mana,
};
const _BONUS_STATS: BasicStats<f32> = bonus_stats!(BasicStats::<f32>(_CURRENT_STATS, _BASE_STATS) {
    attack_damage,
    armor,
    magic_resist,
    health,
    mana
});
const _ABILITY_LEVELS: AbilityLevels = AbilityLevels {
    q: 5,
    w: 5,
    e: 5,
    r: 3,
};
const _ADAPTATIVE_TYPE: AdaptativeType = AdaptativeType::Magic;
const _SELF_STATE: SelfState = SelfState {
    ability_levels: _ABILITY_LEVELS,
    current_stats: _CURRENT_STATS,
    bonus_stats: _BONUS_STATS,
    base_stats: _BASE_STATS,
    level: _LEVEL,
    adaptative_type: _ADAPTATIVE_TYPE,
};
const _RESIST_SHRED: ResistShred = ResistShred {
    armor_penetration_flat: _CURRENT_STATS.armor_penetration_flat,
    armor_penetration_percent: _CURRENT_STATS.armor_penetration_percent,
    magic_penetration_flat: _CURRENT_STATS.magic_penetration_flat,
    magic_penetration_percent: _CURRENT_STATS.magic_penetration_percent,
};
const _E_BASE_STATS: SimpleStats<f32> = SimpleStats::base_stats(ChampionId::Aatrox, 18, false);
const _ENEMY_FULL_STATE: EnemyFullState = get_enemy_state(
    EnemyState {
        base_stats: _E_BASE_STATS,
        items: &[],
        stacks: 0,
        champion_id: ChampionId::Aatrox,
        earth_dragons: 0,
        level: 18,
        item_exceptions: &[],
    },
    _RESIST_SHRED,
    false,
);

const _MAGIC_VALUES: ResistValue = RiotFormulas::real_resist(
    // RESIST_SHRED.magic_penetration_percent,
    // RESIST_SHRED.magic_penetration_flat,
    50.0,
    300.0,
    _E_BASE_STATS.magic_resist,
    false,
);

const _EVAL_CONTEXT: EvalContext = get_eval_ctx(&_SELF_STATE, &_ENEMY_FULL_STATE);
const _ABILITIES: usize = _CHAMPION_ID.number_of_spells();
const _DAMAGES: ConstDamages<_ABILITIES, 0, 0> = const_get_damages(
    &_EVAL_CONTEXT,
    AttackType::Ranged,
    _CHAMPION_ID,
    [],
    [],
    Modifiers::default(),
);

const _S: f32 = RiotFormulas::percent_value(&[30.0, 30.0]);

#[cfg(test)]
mod tests {
    use crate::{Dragons, InputActivePlayer, InputGame, InputMinData, math_test::*};
    extern crate std;
    use alloc::{boxed::Box, format, vec::Vec};
    use std::fs::write;
    use tutorlolv2_gen::NEEKO;

    #[test]
    fn test_calc() {
        let game = InputGame {
            active_player: InputActivePlayer {
                abilities: _ABILITY_LEVELS,
                runes: Default::default(),
                rune_exceptions: Default::default(),
                data: InputMinData {
                    stats: _CURRENT_STATS.into(),
                    level: _LEVEL,
                    item_exceptions: Default::default(),
                    champion_id: _CHAMPION_ID,
                    items: Default::default(),
                    stacks: 0,
                    infer_stats: false,
                    is_mega_gnar: false,
                },
            },
            enemy_players: Box::from([InputMinData {
                stats: _E_BASE_STATS.into(),
                level: _LEVEL,
                item_exceptions: Default::default(),
                champion_id: ChampionId::Aatrox,
                items: Default::default(),
                stacks: 0,
                infer_stats: false,
                is_mega_gnar: false,
            }]),
            dragons: Dragons::default(),
        };
        let data = crate::calculator(game);
        write("calc.txt", format!("{data:#?}")).unwrap();
    }

    #[test]
    fn print_const() {
        let metadata = NEEKO.metadata;
        let d = _DAMAGES.abilities;
        let mut s = Vec::new();
        for i in 0..metadata.len() {
            let meta = metadata[i].kind;
            let damage = d[i];
            s.push(format!("{meta:?}: {damage}"));
        }
        write("test.txt", format!("{s:#?}\n{_EVAL_CONTEXT:#?}")).unwrap()
    }
}
