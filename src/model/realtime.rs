use super::{
    SIZE_ABILITIES, SIZE_ENEMIES_EXPECTED, WrapSetU32,
    base::{Attacks, BasicStats, DamageLike, DragonMultipliers, Stats},
};
use crate::{SIZE_DAMAGING_ITEMS, SIZE_SIMULATED_ITEMS, model::base::InstanceDamage};
use bincode::Encode;
use internal_comptime::{AbilityLike, AdaptativeType, ChampionId, ItemId, Position, RuneId};
use smallvec::SmallVec;

#[derive(Encode)]
pub struct SimulatedDamages {
    pub attacks: Attacks,
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<SIZE_DAMAGING_ITEMS, ItemId>,
    pub runes: DamageLike<3, RuneId>,
}

#[derive(Encode)]
pub struct Damages {
    pub attacks: Attacks,
    pub abilities: DamageLike<SIZE_ABILITIES, AbilityLike>,
    pub items: DamageLike<5, ItemId>,
    pub runes: DamageLike<3, RuneId>,
    pub compared_items: SmallVec<[(ItemId, SimulatedDamages); SIZE_SIMULATED_ITEMS]>,
}

#[derive(Encode)]
pub struct CurrentPlayer<'a> {
    pub damaging_items: WrapSetU32,
    pub damaging_runes: WrapSetU32,
    pub riot_id: &'a str,
    pub level: u8,
    pub team: Team,
    pub adaptative_type: AdaptativeType,
    pub position: Position,
    pub champion_id: ChampionId,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: Stats,
}

#[derive(Encode)]
pub struct GameInformation {
    pub game_time: f32,
    pub map_number: u8,
}

#[derive(Encode)]
pub enum Team {
    Blue,
    Red,
}

impl Team {
    pub fn from_raw(raw: &str) -> Self {
        match raw {
            "ORDER" => Team::Blue,
            "CHAOS" => Team::Red,
            _ => Team::Blue,
        }
    }
}

#[derive(Encode)]
pub struct Enemy<'a> {
    pub riot_id: &'a str,
    pub team: Team,
    pub level: u8,
    pub position: Position,
    pub damages: Damages,
    pub base_stats: BasicStats,
    pub bonus_stats: BasicStats,
    pub current_stats: BasicStats,
    pub real_armor: f32,
    pub real_magic_resist: f32,
}

#[derive(Encode)]
pub struct Scoreboard<'a> {
    pub assists: u16,
    pub creep_score: u16,
    pub deaths: u16,
    pub kills: u16,
    pub riot_id: &'a str,
    pub champion_id: ChampionId,
    pub position: Position,
}

#[derive(Encode)]
pub struct Realtime<'a> {
    pub current_player: CurrentPlayer<'a>,
    pub enemies: SmallVec<[(ChampionId, Enemy<'a>); SIZE_ENEMIES_EXPECTED]>,
    pub game_information: GameInformation,
    pub scoreboard: Scoreboard<'a>,
    pub enemy_dragon_multipliers: DragonMultipliers,
    pub ally_dragon_multipliers: DragonMultipliers,
}

impl Realtime<'_> {
    pub fn bincode_size(&self) -> usize {
        let sc = &self.scoreboard;
        let cp = &self.current_player;
        let en = &self.enemies;
        141 + sc.riot_id.size()
            + sc.kills.size()
            + sc.deaths.size()
            + sc.assists.size()
            + sc.creep_score.size()
            + cp.damaging_items.size()
            + cp.damaging_runes.0.len()
            + cp.riot_id.size()
            + en.len().size()
            + en.iter()
                .map(|(_, e)| {
                    e.riot_id.size()
                        + 96
                        + e.damages.abilities.size()
                        + e.damages.items.size()
                        + e.damages.runes.size()
                        + e.damages.compared_items.size()
                })
                .sum::<usize>()
    }
}

macro_rules! enc {
    ($v:expr) => {
        bincode::encode_to_vec(&$v, bincode::config::standard()).unwrap()
    };
}

#[test]
fn test_bincode() {
    let data = std::fs::read("serde_test.json").unwrap();
    let parsed = serde_json::from_slice(&data).unwrap();
    let game = crate::services::realtime::realtime(&parsed).unwrap();

    macro_rules! size {
        ($field:ident, $len:expr) => {
            debug_assert_eq!(enc!(&game.$field).len(), $len);
        };
    }
    size!(game_information, 5);
    size!(ally_dragon_multipliers, size_of::<DragonMultipliers>());
    size!(enemy_dragon_multipliers, size_of::<DragonMultipliers>());

    debug_assert_eq!(enc!(game).len(), game.bincode_size());

    size!(scoreboard, {
        let sc = &game.scoreboard;
        sc.riot_id.size()
            + sc.kills.size()
            + sc.deaths.size()
            + sc.assists.size()
            + sc.creep_score.size()
            + 2
    });

    let cp = &game.current_player;
    debug_assert_eq!(enc!(cp.damaging_items).len(), cp.damaging_items.size());
    size!(current_player, {
        cp.damaging_items.size() + 110 + cp.damaging_runes.0.len() + cp.riot_id.size()
    });
    size!(enemies, {
        game.enemies.len().size()
            + game
                .enemies
                .iter()
                .map(|(_, e)| {
                    e.riot_id.size()
                        + 96
                        + e.damages.abilities.size()
                        + e.damages.items.size()
                        + e.damages.runes.size()
                        + e.damages.compared_items.size()
                })
                .sum::<usize>()
    });
    let e1 = &game.enemies[1].1;

    macro_rules! compare {
        ($t:ident) => {
            println!(
                "Variant: {}, position: {}, size encoded: {}, size_.size(): {}",
                stringify!($t),
                ItemId::$t as usize,
                enc!(ItemId::$t).len(),
                ItemId::$t.size()
            )
        };
    }

    compare!(AbyssalMask);
    compare!(Perplexity);
    compare!(SanguineGift);
    compare!(StaffofFlowingWater);
    compare!(StatBonus);
    compare!(StatikkShiv);
    compare!(VampiricScepter);
    compare!(Zeal);

    debug_assert_eq!(enc!(e1.riot_id).len(), e1.riot_id.size());
    debug_assert_eq!(
        enc!(e1.damages.abilities).len(),
        e1.damages.abilities.size()
    );
    debug_assert_eq!(enc!(e1.damages.items).len(), e1.damages.items.size());
    debug_assert_eq!(enc!(e1.damages.runes).len(), e1.damages.runes.size());
    debug_assert_eq!(
        enc!(e1.damages.compared_items).len(),
        e1.damages.compared_items.size()
    );
}

#[inline(always)]
fn size_v(v: u32) -> usize {
    if v <= 251 {
        1
    } else if v <= 0xFFFF {
        3
    } else {
        5
    }
}

trait Sizer {
    fn size(&self) -> usize;
}

macro_rules! impl_sizer {
    ($($t:ty),*) => {
        $(
            impl Sizer for $t {
                fn size(&self) -> usize {
                    size_v(*self as u32)
                }
            }
        )+
    };
    (const $t:ty, $ratio:literal) => {
        impl<const N: usize> Sizer for SmallVec<[($t, InstanceDamage); N]> {
            fn size(&self) -> usize {
                let len = self.len();
                len.size() + len * $ratio
            }
        }
    };
}

impl<const N: usize> Sizer for SmallVec<[(ItemId, InstanceDamage); N]> {
    fn size(&self) -> usize {
        self.len().size() + self.iter().map(|x| x.0.size() + 9).sum::<usize>()
    }
}

impl<const N: usize> Sizer for SmallVec<[(ItemId, SimulatedDamages); N]> {
    fn size(&self) -> usize {
        self.len().size()
            + self
                .iter()
                .map(|(k, v)| {
                    debug_assert_eq!(enc!(v.abilities).len(), v.abilities.size());
                    debug_assert_eq!(enc!(v.items).len(), v.items.size());
                    debug_assert_eq!(enc!(v.runes).len(), v.runes.size());
                    debug_assert_eq!(enc!(k).len(), k.size());
                    debug_assert_eq!(enc!(v.attacks).len(), 24);
                    k.size() + 24 + v.abilities.size() + v.items.size() + v.runes.size()
                })
                .sum::<usize>()
    }
}

impl_sizer!(u8, u16, u32, u64, usize, ItemId);
impl_sizer!(const ChampionId, 10);
impl_sizer!(const RuneId, 10);
impl_sizer!(const AbilityLike, 11);

impl Sizer for WrapSetU32 {
    fn size(&self) -> usize {
        self.0.len().size() + self.0.iter().map(|x| x.size()).sum::<usize>()
    }
}

impl Sizer for str {
    fn size(&self) -> usize {
        let len = self.len();
        len + size_v(len as u32)
    }
}
