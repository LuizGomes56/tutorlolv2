use crate::model::Sizer;

use super::{
    SIZE_ABILITIES, SIZE_ITEMS_EXPECTED, SIZE_RUNES_EXPECTED,
    base::{AbilityLevels, Attacks, BasicStats, DamageLike, MonsterDamages, Stats},
    functions::WrapSetU32,
};
use bincode::{Decode, Encode};
use internal_comptime::{AbilityLike, AdaptativeType, ChampionId, ItemId, RuneId};
use smallvec::SmallVec;

#[derive(Encode)]
pub struct OutputCurrentPlayer {
    pub champion_id: ChampionId,
    pub damaging_items: WrapSetU32,
    pub damaging_runes: WrapSetU32,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Encode)]
pub struct CalculatorDamages {
    pub attacks: Attacks,
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<5, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct OutputEnemy {
    pub level: u8,
    pub damages: CalculatorDamages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f32,
    pub real_magic_resist: f32,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: MonsterDamages,
    pub tower_damages: [f32; 6],
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[(ChampionId, OutputEnemy); 1]>,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub champion_id: ChampionId,
    pub champion_stats: Stats,
    pub abilities: AbilityLevels,
    pub items: SmallVec<[ItemId; SIZE_ITEMS_EXPECTED]>,
    pub runes: SmallVec<[RuneId; SIZE_RUNES_EXPECTED]>,
    pub level: u8,
    pub stacks: u32,
    pub infer_stats: bool,
}

#[derive(Decode)]
pub struct InputEnemyPlayers {
    pub champion_id: ChampionId,
    pub items: SmallVec<[ItemId; SIZE_ITEMS_EXPECTED]>,
    pub level: u8,
    pub stats: BasicStats,
    pub infer_stats: bool,
    pub stacks: u32,
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputEnemyPlayers; 1]>,
    pub ally_earth_dragons: u8,
    pub ally_fire_dragons: u8,
    pub enemy_earth_dragons: u8,
    // #![todo] well defined struct instead of a Map or Vec
    // pub stack_exceptions: SmallVec<[(u32, u32); SIZE_STACK_EXCEPTIONS]>,
}

impl Sizer for OutputGame {
    fn size(&self) -> usize {
        use super::functions::enc;
        unsafe {
            let md = &self.monster_damages.get_unchecked(0);
            let cp = &self.current_player;
            let el = self.enemies.len();
            let mut sum =
                301 + 7 * {
                    let la = md.abilities.len();
                    let li = md.items.len();
                    la.size() + li.size() + 9 * (la + li)
                } + cp.damaging_items.size()
                    + cp.damaging_runes.0.len()
                    + el.size();
            if el > 0 {
                let e0 = &self.enemies.get_unchecked(0).1;
                debug_assert_eq!(
                    enc!(e0).len(),
                    93 + e0.damages.abilities.size()
                        + e0.damages.items.size()
                        + e0.damages.runes.size()
                );
                sum += el
                    * (93
                        + e0.damages.abilities.size()
                        + e0.damages.items.size()
                        + e0.damages.runes.size())
            }
            sum
        }
    }
}

#[test]
fn test_bincode_size_c() {
    use super::functions::enc;
    let game = crate::services::calculator::calculator(InputGame {
        active_player: InputActivePlayer {
            champion_id: ChampionId::Aatrox,
            champion_stats: Stats {
                ability_power: 1000.0,
                armor: 1000.0,
                armor_penetration_flat: 1000.0,
                armor_penetration_percent: 1000.0,
                attack_damage: 1000.0,
                attack_range: 1000.0,
                attack_speed: 1000.0,
                crit_chance: 1000.0,
                crit_damage: 1000.0,
                current_health: 1000.0,
                magic_penetration_flat: 1000.0,
                magic_penetration_percent: 1000.0,
                magic_resist: 1000.0,
                max_health: 1000.0,
                max_mana: 1000.0,
                current_mana: 1000.0,
            },
            abilities: AbilityLevels {
                q: 5,
                w: 5,
                e: 5,
                r: 3,
            },
            items: Default::default(),
            runes: Default::default(),
            level: 15,
            stacks: 0,
            infer_stats: true,
        },
        enemy_players: SmallVec::from_vec(vec![InputEnemyPlayers {
            champion_id: ChampionId::Gwen,
            items: Default::default(),
            level: 15,
            stats: BasicStats {
                armor: 1000.0,
                health: 1000.0,
                attack_damage: 1000.0,
                magic_resist: 1000.0,
                mana: 1000.0,
            },
            infer_stats: true,
            stacks: 0,
        }]),
        ally_earth_dragons: 0,
        ally_fire_dragons: 0,
        enemy_earth_dragons: 0,
    })
    .unwrap();
    assert_eq!(enc!(game).len(), game.size());
}
