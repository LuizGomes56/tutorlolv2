use super::riot::RiotChampionStats;
use bincode::{Decode, Encode};
use smallvec::SmallVec;
use tutorlolv2_gen::{
    AbilityLike, AdaptativeType, Attrs, ChampionId, DamageType, ItemId, Position, RuneId,
    SIMULATED_ITEMS,
};

pub const L_SIML: usize = SIMULATED_ITEMS.len();
pub const L_RUNE: usize = 2;
pub const L_ITEM: usize = 4;
pub const L_ABLT: usize = 7;
pub const L_TEAM: usize = 5;
pub const L_PLYR: usize = L_TEAM << 1;

#[derive(Encode)]
pub enum Team {
    Blue,
    Red,
}

#[derive(Encode)]
pub struct RangeDamage {
    pub minimum_damage: f32,
    pub maximum_damage: f32,
}

#[derive(Encode, Decode)]
pub struct StatsI32 {
    pub ability_power: i32,
    pub armor: i32,
    pub armor_penetration_flat: i32,
    pub armor_penetration_percent: i32,
    pub attack_damage: i32,
    pub attack_range: i32,
    pub attack_speed: i32,
    pub crit_chance: i32,
    pub crit_damage: i32,
    pub current_health: i32,
    pub magic_penetration_flat: i32,
    pub magic_penetration_percent: i32,
    pub magic_resist: i32,
    pub max_health: i32,
    pub max_mana: i32,
    pub current_mana: i32,
}

pub struct BasicStatsF32 {
    pub armor: f32,
    pub health: f32,
    pub attack_damage: f32,
    pub magic_resist: f32,
    pub mana: f32,
}

#[derive(Encode)]
pub struct BasicStatsI32 {
    pub armor: i32,
    pub health: i32,
    pub attack_damage: i32,
    pub magic_resist: i32,
    pub mana: i32,
}

#[derive(Encode, Decode)]
pub struct AbilityLevels {
    pub q: u8,
    pub w: u8,
    pub e: u8,
    pub r: u8,
}

#[derive(Decode, Default)]
pub struct Dragons {
    pub earth: u8,
    pub fire: u8,
}

#[derive(Encode)]
pub struct Attacks {
    pub basic_attack: RangeDamage,
    pub critical_strike: RangeDamage,
    pub onhit_damage: RangeDamage,
}

#[derive(Encode)]
pub struct TypeMetadata<T, U> {
    pub attack_damage: DamageType,
    pub kind: T,
    pub meta: U,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[Enemy<'a>; L_TEAM]>,
    pub scoreboard: SmallVec<[Scoreboard<'a>; L_PLYR]>,
    pub abilities: SmallVec<[TypeMetadata<AbilityLike, Attrs>; L_ABLT]>,
    pub items: SmallVec<[TypeMetadata<ItemId, u8>; L_ITEM]>,
    pub runes: SmallVec<[TypeMetadata<RuneId, ()>; L_RUNE]>,
    pub siml_items: [TypeMetadata<ItemId, u8>; L_SIML],
    pub game_time: i32,
    pub ability_levels: AbilityLevels,
}

#[derive(Encode)]
pub struct Scoreboard<'a> {
    pub riot_id: &'a str,
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u8,
    pub kills: u8,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Encode)]
pub struct CurrentPlayer<'a> {
    pub riot_id: &'a str,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub current_stats: StatsI32,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
}

#[derive(Encode, Decode)]
pub struct SimpleStats {
    pub armor: i32,
    pub health: i32,
    pub magic_resist: i32,
}

#[derive(Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub damages: Damages,
    pub siml_items: [Damages; L_SIML],
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: SimpleStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
    pub team: Team,
    pub position: Position,
}

#[derive(Encode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: SmallVec<[RangeDamage; L_ABLT]>,
    pub items: SmallVec<[RangeDamage; L_ITEM]>,
    pub runes: SmallVec<[RangeDamage; L_RUNE]>,
}

#[derive(Decode)]
pub struct StackExceptionKind<T> {
    pub kind: T,
    pub stacks: u16,
    pub offset: u8,
}

#[derive(Decode)]
pub enum StackException {
    Item(StackExceptionKind<ItemId>),
    Rune(StackExceptionKind<RuneId>),
    Champion(StackExceptionKind<ChampionId>),
}

#[derive(Decode)]
pub struct InputGame {
    pub active_player: InputActivePlayer,
    pub enemy_players: SmallVec<[InputEnemyPlayers; 1]>,
    pub stack_exceptions: SmallVec<[StackException; 5]>,
    pub ally_dragons: Dragons,
    pub enemy_earth_dragons: u8,
    pub _padding: u32,
}

#[derive(Decode)]
pub struct InputActivePlayer {
    pub champion_stats: StatsI32,
    pub runes: SmallVec<[RuneId; L_RUNE]>,
    pub abilities: AbilityLevels,
    pub data: InputMinData,
}

#[derive(Decode)]
pub struct InputMinData {
    pub items: SmallVec<[ItemId; L_ITEM]>,
    pub stacks: u32,
    pub level: u8,
    pub infer_stats: bool,
    pub attack_form: bool,
    pub champion_id: ChampionId,
}

#[derive(Decode)]
pub struct InputEnemyPlayers {
    pub stats: SimpleStats,
    pub data: InputMinData,
}

#[derive(Encode)]
pub struct OutputEnemy {
    pub damages: Damages,
    pub base_stats: SimpleStats,
    pub bonus_stats: SimpleStats,
    pub current_stats: SimpleStats,
    pub real_armor: i32,
    pub real_magic_resist: i32,
    pub level: u8,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub struct OutputCurrentPlayer {
    pub current_stats: StatsI32,
    pub base_stats: BasicStatsI32,
    pub bonus_stats: BasicStatsI32,
    pub level: u8,
    pub adaptative_type: AdaptativeType,
    pub champion_id: ChampionId,
}

#[derive(Encode)]
pub struct MonsterDamage {
    pub attacks: Attacks,
    pub abilities: SmallVec<[RangeDamage; L_ABLT]>,
    pub items: SmallVec<[RangeDamage; L_ITEM]>,
}

#[derive(Encode)]
pub struct OutputGame {
    pub monster_damages: SmallVec<[MonsterDamage; 7]>,
    pub current_player: OutputCurrentPlayer,
    pub enemies: SmallVec<[OutputEnemy; 1]>,
    pub tower_damages: [i32; 6],
}

impl From<StatsI32> for RiotChampionStats {
    fn from(value: StatsI32) -> Self {
        Self {
            ability_power: value.ability_power as f32,
            armor: value.armor as f32,
            armor_penetration_flat: value.armor_penetration_flat as f32,
            armor_penetration_percent: value.armor_penetration_percent as f32,
            attack_damage: value.attack_damage as f32,
            attack_range: value.attack_range as f32,
            attack_speed: value.attack_speed as f32,
            crit_chance: value.crit_chance as f32,
            crit_damage: value.crit_damage as f32,
            current_health: value.current_health as f32,
            magic_penetration_flat: value.magic_penetration_flat as f32,
            magic_penetration_percent: value.magic_penetration_percent as f32,
            magic_resist: value.magic_resist as f32,
            health: value.max_health as f32,
            mana: value.max_mana as f32,
            current_mana: value.current_mana as f32,
        }
    }
}
