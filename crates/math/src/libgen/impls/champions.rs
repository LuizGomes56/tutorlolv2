#[cfg(feature = "engine")]
use {crate::champions_code::ability_const_eval, types::Ctx};

use {
    crate::{
        ChampionId, ItemId, MapStats, RuneId, WikiStats,
        champions_code::{RECOMMENDED_ITEMS, RECOMMENDED_RUNES},
    },
    strum::{EnumCount, VariantArray},
    types::{
        AbilityId, AbilityName, AdaptiveType, AttackType, GameMap, MapMode, Position, TypeMetadata,
    },
};

impl ChampionId {
    pub const RECOMMENDED_ITEMS: &[[&[ItemId]; Position::COUNT]; ChampionId::COUNT] =
        &RECOMMENDED_ITEMS;

    pub const RECOMMENDED_RUNES: &[[&[RuneId]; Position::COUNT]; ChampionId::COUNT] =
        &RECOMMENDED_RUNES;

    pub const TOTAL_ABILITIES: usize = {
        let mut i = 0;
        let mut sum = 0;

        while i < Self::COUNT {
            let data = Self::DATA[i];
            sum += data.metadata.len();
            i += 1;
        }

        sum
    };

    pub const POSITIONS: [&[Position]; Self::COUNT] = {
        let mut i = 0;
        let mut result = [&[] as &[_]; _];

        while i < Self::COUNT {
            let champion = Self::DATA[i];
            result[i] = champion.positions;
            i += 1;
        }

        result
    };

    pub const IRML: usize = {
        let mut i = 0;
        let mut max = 0;

        while i < Self::COUNT {
            let mut j = 0;
            let champion_id = Self::VARIANTS[i];

            while j < Position::COUNT {
                let position = Position::VARIANTS[j];
                let data = champion_id.recommended_items(position);
                if data.len() > max {
                    max = data.len();
                }
                j += 1;
            }
            i += 1;
        }
        max
    };

    pub const fn abilities(&self) -> &'static [TypeMetadata<AbilityId>] {
        self.data().metadata
    }

    pub const fn stats(&self) -> &'static WikiStats {
        &self.data().stats
    }

    /// Stats of this champion on `game_map`
    #[inline(always)]
    pub const fn stats_on(&self, game_map: GameMap) -> &'static WikiStats {
        match self.map_stats(game_map) {
            Some(map_stats) => &map_stats.stats,
            None => self.stats(),
        }
    }

    /// Stats that `game_map` changes. Maps without a mode change none, and nearly
    /// every game is on Summoner's Rift, so the search is cold
    #[inline(always)]
    pub const fn map_stats(&self, game_map: GameMap) -> Option<&'static MapStats> {
        // Maps without a mode, as Summoner's Rift and unknown maps, change nothing
        if game_map.mode() as u8 == MapMode::Default as u8 {
            return None;
        }

        self.find_map_stats(game_map)
    }

    /// Search of [`Self::map_stats`], kept out of line so that the check of
    /// Summoner's Rift inlines into the callers of every enemy
    #[cold]
    #[inline(never)]
    const fn find_map_stats(&self, game_map: GameMap) -> Option<&'static MapStats> {
        let list = self.data().map_stats;
        let mut i = 0;

        while i < list.len() {
            let mut j = 0;

            while j < list[i].maps.len() {
                if list[i].maps[j] as u8 == game_map as u8 {
                    return Some(&list[i]);
                }

                j += 1;
            }

            i += 1;
        }

        None
    }

    pub const fn number_of_abilities(&self) -> usize {
        self.data().metadata.len()
    }

    pub const fn adaptive_type(&self) -> AdaptiveType {
        self.data().adaptive_type
    }

    pub const fn ability_ids<const N: usize>(&self) -> [AbilityId; N] {
        let mut result = [AbilityId::P(AbilityName::Void); _];
        let mut i = 0;

        assert!(N == self.number_of_abilities());

        while i < N {
            result[i] = self.abilities()[i].kind;
            i += 1;
        }

        result
    }

    pub const fn recommended_items(&self, position: Position) -> &'static [ItemId] {
        Self::RECOMMENDED_ITEMS[self.index()][position.index()]
    }

    pub const fn recommended_runes(&self, position: Position) -> &'static [RuneId] {
        Self::RECOMMENDED_RUNES[self.index()][position.index()]
    }

    pub const fn positions(&self) -> &'static [Position] {
        self.data().positions
    }

    pub const fn main_position(&self) -> Position {
        self.positions()[0]
    }

    pub const fn attack_type(&self) -> AttackType {
        self.data().attack_type
    }

    pub const fn index_of_ability(&self, ability_id: AbilityId) -> Option<usize> {
        let mut i = 0;

        while i < self.number_of_abilities() {
            if self.abilities()[i].kind.const_eq(ability_id) {
                return Some(i);
            }

            i += 1;
        }

        None
    }

    pub const fn metadata(&self) -> &'static [TypeMetadata<AbilityId>] {
        self.data().metadata
    }

    #[cfg(feature = "engine")]
    pub const fn eval(&self, ctx: &Ctx, kind: AbilityId) -> f32 {
        ability_const_eval(*self, ctx, kind)
    }

    #[cfg(feature = "client-data")]
    pub const fn exceptions(&self, ally: bool) -> Option<types::Key> {
        use types::Key;

        match ally {
            true => match self {
                Self::AurelionSol
                | Self::Bard
                | Self::Belveth
                | Self::Graves
                | Self::Hecarim
                | Self::Kalista
                | Self::Kindred
                | Self::Senna
                | Self::Shyvana
                | Self::Sion
                | Self::Smolder
                | Self::Swain
                | Self::Thresh
                | Self::Veigar => Some(Key::P),
                Self::Nasus => Some(Key::Q),
                Self::Darius => Some(Key::E),
                Self::Chogath => Some(Key::R),
                _ => None,
            },
            false => match self {
                Self::Graves | Self::Sion | Self::Swain | Self::Thresh => Some(Key::P),
                Self::Chogath => Some(Key::R),
                _ => None,
            },
        }
    }

    #[cfg(feature = "client-data")]
    pub const fn identifiers(&self) -> &'static [&'static [types::CtxVar]] {
        self.data().identifiers
    }

    #[cfg(any(feature = "yew", feature = "formula-docs"))]
    pub const fn combos(&self) -> &'static [&'static [types::ComboElement]] {
        self.data().combos
    }

    #[cfg(feature = "client-data")]
    pub const fn merge_data(&self) -> &'static [types::MergeData] {
        self.data().merge_data
    }
}
