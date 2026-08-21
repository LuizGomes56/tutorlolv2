use {
    crate::{
        ChampionId, ItemId, RuneId, WikiStats,
        champions_code::{RECOMMENDED_ITEMS, RECOMMENDED_RUNES, ability_const_eval},
    },
    tutorlolv2_types::{
        AbilityId, AbilityName, AdaptiveType, AttackType, ComboElement, Ctx, Key, MergeData,
        Position, TypeMetadata,
    },
};

impl ChampionId {
    pub const RECOMMENDED_ITEMS: &[[&[ItemId]; Position::VARIANTS]; ChampionId::VARIANTS] =
        &RECOMMENDED_ITEMS;
    pub const RECOMMENDED_RUNES: &[[&[RuneId]; Position::VARIANTS]; ChampionId::VARIANTS] =
        &RECOMMENDED_RUNES;

    pub const TOTAL_ABILITIES: usize = {
        let mut i = 0;
        let mut sum = 0;

        while i < Self::VARIANTS {
            let data = Self::DATA[i];
            sum += data.closures.len();
            i += 1;
        }

        sum
    };

    pub const POSITIONS: [&[Position]; Self::VARIANTS] = {
        let mut i = 0;
        let mut result = [&[] as &[_]; _];

        while i < Self::VARIANTS {
            let champion = Self::DATA[i];
            result[i] = champion.positions;
            i += 1;
        }

        result
    };

    pub const IRML: usize = {
        let mut i = 0;
        let mut max = 0;

        while i < Self::VARIANTS {
            let mut j = 0;
            let champion_id = Self::VALUES[i];

            while j < Position::VARIANTS as usize {
                let position = Position::ARRAY[j];
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

    pub const fn exceptions(&self, ally: bool) -> Option<Key> {
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

    #[cfg(feature = "yew")]
    pub const fn identifiers(&self) -> &'static [&'static [tutorlolv2_types::CtxVar]] {
        self.data().identifiers
    }

    pub const fn abilities(&self) -> &'static [TypeMetadata<AbilityId>] {
        self.data().metadata
    }

    pub const fn stats(&self) -> &'static WikiStats {
        &self.data().stats
    }

    pub const fn merge_data(&self) -> &'static [MergeData] {
        self.data().merge_data
    }

    pub const fn number_of_abilities(&self) -> usize {
        self.data().closures.len()
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

    pub const fn combos(&self) -> &'static [&'static [ComboElement]] {
        self.data().combos
    }

    pub const fn index_of_ability(&self, ability_id: AbilityId) -> Option<usize> {
        let mut i = 0;
        while i < self.number_of_abilities() {
            if self.abilities()[i].kind.const_eq(ability_id) {
                return Some(i);
            }
            i += 1;
        }
        return None;
    }

    pub const fn metadata(&self) -> &'static [TypeMetadata<AbilityId>] {
        self.data().metadata
    }

    pub const fn eval(&self, ctx: &Ctx, kind: AbilityId) -> f32 {
        ability_const_eval(*self, ctx, kind)
    }
}
