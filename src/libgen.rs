use crate::{
    bitset,
    bitset::*,
    generated::{champions::*, docs::*, items::*, runes::*, *},
};
use core::{
    any::Any,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::Range,
    str::FromStr,
};
use tutorlolv2_types::*;

pub const fn ignite(level: u8) -> i32 {
    let n = level as i32;
    let nth = if n > 4 { n - 4 } else { 0 };
    70 + 20 * n + 5 * nth
}

impl Item {
    /// Verifies the following conditions
    /// - `tier >= 3`
    /// - `price > 0`
    /// - `len(stats)` > 0
    /// - `purchasable`
    pub const fn is_simulated_item(&self) -> bool {
        let Self {
            purchasable,
            tier,
            price,
            maps,
            metadata: TypeMetadata { kind, .. },
            ..
        } = *self;

        let check = [
            StatName::AbilityPower,
            StatName::AttackDamage,
            StatName::AdaptiveForce,
            StatName::Lethality,
            StatName::ArmorPenetration,
            StatName::MagicPenetration,
        ];

        let mut allow = false;
        let mut i = 0;

        while i < check.len() {
            if kind.has_stat(check[i]) {
                allow = true;
            }

            i += 1;
        }

        tier >= 3 && price > 0 && purchasable && allow && {
            let mut j = 0;
            while j < maps.len() {
                if matches!(maps[j], GameMap::SummonersRift) {
                    return true;
                }
                j += 1;
            }
            false
        }
    }
}

/// Number of items that are compared and obey the rule:
/// - `tier >= 3`
/// - `price > 0`
/// - `len(stats)` > 0
/// - `purchasable`
/// - `maps.summoners_rift`
pub const L_SIML: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < ItemId::VARIANTS {
        if ITEMS_DATA[i].is_simulated_item() {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// Exact number of resistence variations for jungle monsters
pub const L_MSTR: usize = 7;

/// Number of different plates a tower can have. Each tower can have `0..=5` plates
pub const L_TWRD: usize = 6;

/// Stores the simulated items as [`ItemId`], and only those that follow the rules:
/// - `tier >= 3`
/// - `price > 0`
/// - `purchasable`
pub const SIMULATED_ITEMS_ENUM: [ItemId; L_SIML] = {
    let mut result: [ItemId; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;
    let mut j = 0;
    while i < ItemId::VARIANTS {
        if ITEMS_DATA[i].is_simulated_item() {
            result[j] = ItemId::from_repr(i as _).unwrap();
            j += 1;
        }
        i += 1;
    }
    result
};

impl FromStr for ChampionId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CHAMPION_NAME_TO_ID
            .get(s)
            .copied()
            .ok_or("No matches when calling ChampionId::from_str")
    }
}

impl FromStr for ItemId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ITEM_NAME_TO_ID
            .get(s)
            .copied()
            .ok_or("No matches when calling ItemId::from_str")
    }
}

impl FromStr for RuneId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RUNE_NAME_TO_ID
            .get(s)
            .copied()
            .ok_or("No matches when calling RuneId::from_str")
    }
}

/// Contains the metadata of all items that have their stats compared to choose
/// which one is best to buy considering the current game state. See [`TypeMetadata`]
/// for more details
pub const SIMULATED_ITEMS_METADATA: [TypeMetadata<ItemId>; L_SIML] = {
    let mut siml_items = MaybeUninit::<[TypeMetadata<ItemId>; L_SIML]>::uninit();
    let siml_items_ptr = siml_items.as_mut_ptr();
    let mut i = 0;
    while i < L_SIML {
        let item_id = SIMULATED_ITEMS_ENUM[i];
        let TypeMetadata {
            damage_type,
            attributes,
            ..
        } = item_id.metadata();
        unsafe {
            (&raw mut (*siml_items_ptr)[i]).write(TypeMetadata {
                kind: item_id,
                damage_type,
                attributes,
            })
        };
        i += 1;
    }
    unsafe { siml_items.assume_init() }
};

/// Number of runes that can damage enemies. Currently they're generated manually and
/// might be outdated. Also, they're stored in a single `.json` file, instead of containing
/// a dedicated file for each rune
pub const NUMBER_OF_DAMAGING_RUNES: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < RuneId::VARIANTS {
        let rune = RUNES_DATA[i];
        let [mmin, mmax, rmin, rmax] = rune.deals_damage;
        if mmin || mmax || rmin || rmax {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// Number of items that can damage enemies. All items have their own files
/// and access to the `MerakiCdn` collected data, which can be used to create
/// their damage closures and insert in a static variable, replacing the [`zero`] constant
pub const NUMBER_OF_DAMAGING_ITEMS: usize = {
    let mut sum = 0;
    let mut i = 0;
    while i < ItemId::VARIANTS {
        let item = ItemId::from_repr(i as _).unwrap();
        if item.deals_damage() {
            sum += 1;
        }
        i += 1;
    }
    sum
};

/// A constant array of all items that can damage enemies, holding their internal ids,
/// defined by the enum [`ItemId`]
pub const DAMAGING_ITEMS_ARRAY: [ItemId; NUMBER_OF_DAMAGING_ITEMS] = {
    let mut result: [ItemId; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;
    let mut j = 0;

    while i < ItemId::VARIANTS {
        let item = ItemId::from_repr(i as _).unwrap();

        if item.deals_damage() {
            result[j] = item;
            j += 1;
        }

        i += 1;
    }
    result
};

/// A constant array of all runes that can damage enemies, holding their internal ids,
/// defined by the enum [`RuneId`]
pub const DAMAGING_RUNES_ARRAY: [RuneId; NUMBER_OF_DAMAGING_RUNES] = {
    let mut result: [RuneId; _] = unsafe { core::mem::zeroed() };
    let mut i = 0;
    let mut j = 0;

    while i < RuneId::VARIANTS {
        let rune = RUNES_DATA[i];
        let [mmin, mmax, rmin, rmax] = rune.deals_damage;

        if mmin || mmax || rmin || rmax {
            result[j] = RuneId::from_repr(i as _).unwrap();
            j += 1;
        }

        i += 1;
    }
    result
};

pub const DAMAGING_ITEMS: ItemsBitSet = bitset!(DAMAGING_ITEMS_ARRAY);
pub const DAMAGING_RUNES: RunesBitSet = bitset!(DAMAGING_RUNES_ARRAY);

/// Counts how many damaging abilities ewe have across all champions. This is used to
/// determine a proper size of how many abilities we should allow to live in the stack
/// before leaking it to the heap to avoid stack overflows
pub const NUMBER_OF_ABILITIES: usize = {
    let mut i = 0;
    let mut sum = 0;
    while i < ChampionId::VARIANTS {
        let data = CHAMPIONS_DATA[i];
        sum += data.closures.len();
        i += 1;
    }
    sum
};

pub static CHAMPION_POSITIONS: [&[Position]; ChampionId::VARIANTS] = {
    let mut i = 0;
    let mut result = [&[] as &[_]; _];
    while i < ChampionId::VARIANTS {
        let champion = CHAMPIONS_DATA[i];
        result[i] = champion.positions;
        i += 1;
    }
    result
};

const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion_id = ChampionId::VALUES[i];
        let merge_data = champion_id.merge_data();

        let len = champion_id.number_of_abilities();

        assert!(len == champion_id.closures().len());
        assert!(len == champion_id.identifiers().len());

        let mut j = 0;
        while j < merge_data.len() {
            let m = &merge_data[j];
            assert!((m.min as usize) < len);
            assert!((m.max as usize) < len);
            assert!(m.min < m.max);
            if j + 1 < merge_data.len() {
                let a = &merge_data[j];
                let b = &merge_data[j + 1];
                assert!(a.max < b.max);
            }
            j += 1;
        }

        let mut k = 0;
        let combos = champion_id.combos();

        while k < combos.len() {
            let combo = combos[k];
            let mut l = 0;

            while l < combo.len() {
                let element = combo[l];

                if let ComboElement::Ability(ability_id) = element {
                    assert!(champion_id.index_of_ability(ability_id).is_some());
                }

                l += 1;
            }

            k += 1;
        }

        i += 1;
    }
};

/// Assert there were no undefined behavior while creating [`CHAMPION_POSITIONS`]
const _: () = {
    let mut i = 0;
    while i < ChampionId::VARIANTS {
        let champion = CHAMPIONS_DATA[i].positions;
        let position = CHAMPION_POSITIONS[i];
        assert!(!position.is_empty());
        assert!(champion.len() == position.len());
        let mut j = 0;
        while j < champion.len() {
            let pos_a = champion[j];
            let pos_b = position[j];
            assert!(pos_a.index() == pos_b.index());
            j += 1;
        }
        i += 1;
    }
};

impl TryFrom<&str> for ChampionId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CHAMPION_NAME_TO_ID
            .get(value)
            .copied()
            .ok_or("Failed to convert &str to ChampionId on TryFrom trait")
    }
}

impl ChampionId {
    pub const CLOSURES: &[&[Range<usize>]; Self::VARIANTS] = &ABILITY_CLOSURES;
    pub const ABILITIES: &[&[Range<usize>]; Self::VARIANTS] = &ABILITY_FORMULAS;
    pub const GENERATORS: &[Range<usize>; Self::VARIANTS] = &CHAMPION_GENERATOR;

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
                ChampionId::AurelionSol
                | ChampionId::Bard
                | ChampionId::Belveth
                | ChampionId::Graves
                | ChampionId::Hecarim
                | ChampionId::Kalista
                | ChampionId::Kindred
                | ChampionId::Senna
                | ChampionId::Shyvana
                | ChampionId::Sion
                | ChampionId::Smolder
                | ChampionId::Swain
                | ChampionId::Thresh
                | ChampionId::Veigar => Some(Key::P),
                ChampionId::Nasus => Some(Key::Q),
                ChampionId::Darius => Some(Key::E),
                ChampionId::Chogath => Some(Key::R),
                _ => None,
            },
            false => match self {
                Self::Graves | Self::Sion | Self::Swain | Self::Thresh => Some(Key::P),
                Self::Chogath => Some(Key::R),
                _ => None,
            },
        }
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
        self.closures().len()
    }

    pub const fn adaptive_type(&self) -> AdaptiveType {
        self.data().adaptive_type
    }

    pub const fn ability_ids<const N: usize>(&self) -> [AbilityId; N] {
        let mut i = 0;

        assert!(N == self.number_of_abilities());

        let mut result = [AbilityId::P(AbilityName::Void); _];
        while i < N {
            result[i] = self.abilities()[i].kind;
            i += 1;
        }
        result
    }

    pub const fn recommended_items(&self, position: Position) -> &'static [ItemId] {
        RECOMMENDED_ITEMS[self.index()][position.index()]
    }

    pub const fn recommended_runes(&self, position: Position) -> &'static [RuneId] {
        RECOMMENDED_RUNES[self.index()][position.index()]
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

    pub const fn closures(&self) -> &'static [Range<usize>] {
        Self::CLOSURES[self.index()]
    }

    pub const fn ability_formulas(&self) -> &'static [Range<usize>] {
        ABILITY_FORMULAS[self.index()]
    }

    pub const fn get_ability_formula(&self, index: usize) -> &'static Range<usize> {
        &self.ability_formulas()[index]
    }

    pub const fn get_ability_closure(&self, index: usize) -> &'static Range<usize> {
        &self.closures()[index]
    }

    pub const fn identifiers(&self) -> &'static [&'static [CtxVar]] {
        self.data().identifiers
    }

    pub const fn get_ability_idents(&self, index: usize) -> &'static [CtxVar] {
        &self.identifiers()[index]
    }

    pub const fn generator(&self) -> &'static Range<usize> {
        &CHAMPION_GENERATOR[self.index()]
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

macro_rules! impl_item_filters {
    ($($name:ident),+$(,)?) => {
        impl ItemId {
            pastey::paste! {
                pub const FILTERS: [&[Self]; StatName::VARIANTS] = [
                    $(
                        &Self::[<ITEMS_WITH_ $name:snake:upper>],
                    )+
                ];

                $(
                    pub const [<ITEMS_WITH_ $name:snake:upper>]: [Self; Self::count_variants(StatName::$name)] =
                        Self::find_variants(StatName::$name);
                )+
            }
        }
    };
}

impl_item_filters! {
    AbilityHaste,
    AbilityPower,
    AdaptiveForce,
    Armor,
    ArmorPenetration,
    AttackDamage,
    AttackSpeed,
    BaseHealthRegen,
    BaseManaRegen,
    CritChance,
    CritDamage,
    GoldPer10Seconds,
    HealAndShieldPower,
    Health,
    Lethality,
    LifeSteal,
    MagicPenetration,
    MagicPenetrationPercent,
    MagicResist,
    Mana,
    MoveSpeed,
    MoveSpeedPercent,
    Omnivamp,
    Tenacity,
}

const _: () = {
    let mut i = 0;
    while i < ItemId::VARIANTS {
        let item = ItemId::VALUES[i];
        if item.deals_max_damage() {
            assert!(item.deals_damage());
        }
        i += 1;
    }

    let mut j = 0;
    while j < RuneId::VARIANTS {
        let rune = RuneId::VALUES[j];
        if rune.deals_max_damage() {
            assert!(rune.deals_damage());
        }
        j += 1;
    }
};

impl ItemId {
    pub const CLOSURES: &[[[Range<usize>; 2]; 2]; Self::VARIANTS] = &ITEM_CLOSURES;
    pub const GENERATORS: &[Range<usize>; Self::VARIANTS] = &ITEM_GENERATOR;
    pub const RIOT_IDS: [u32; Self::VARIANTS] = {
        let mut result = [0; _];
        let mut i = 0;
        while i < Self::VARIANTS {
            let value = Self::VALUES[i];
            result[i] = value.to_riot_id();
            i += 1;
        }
        result
    };
    pub const SIML: &[Self; L_SIML] = &SIMULATED_ITEMS_ENUM;

    pub const ALLY_EXCEPTIONS: [Self; 8] = [
        Self::DarkSeal,
        Self::Dragonheart,
        Self::DemonKingsCrown,
        Self::RiteOfRuin,
        Self::MejaisSoulstealer,
        Self::Hubris,
        Self::BloodlettersCurse,
        Self::BlackCleaver,
    ];

    pub const ENEMY_EXCEPTIONS: [Self; 3] =
        [Self::Dragonheart, Self::DemonKingsCrown, Self::BlackCleaver];

    pub const SIZE_OF_EXCEPTIONS: usize = max_usize(
        bitset_size(bitset!(ItemId::ALLY_EXCEPTIONS => [usize])),
        bitset_size(bitset!(ItemId::ENEMY_EXCEPTIONS => [usize])),
    );

    pub const fn is_siml(&self) -> bool {
        let mut i = 0;
        while i < L_SIML {
            if self.index() == Self::SIML[i].index() {
                return true;
            }
            i += 1;
        }
        false
    }

    pub const fn indexof_siml(index: usize) -> Option<Self> {
        let mut i = 0;
        while i < L_SIML {
            if index == Self::SIML[i].index() {
                return Some(Self::SIML[i]);
            }
            i += 1;
        }
        None
    }

    pub const fn damage_type(&self) -> DamageType {
        self.metadata().damage_type
    }

    pub const fn exceptions(ally: bool) -> ItemsExcSet {
        match ally {
            true => bitset!(ItemId::ALLY_EXCEPTIONS),
            false => bitset!(ItemId::ENEMY_EXCEPTIONS),
        }
    }

    pub const fn maps(&self) -> &'static [GameMap] {
        self.data().maps
    }

    pub const fn has_map(&self, game_map: GameMap) -> bool {
        let stats = self.maps();
        let mut i = 0;

        while i < stats.len() {
            if stats[i] as u8 == game_map as u8 {
                return true;
            }
            i += 1;
        }

        false
    }

    pub const fn has_stat(&self, stat_name: StatName) -> bool {
        let stats = self.stats();
        let mut i = 0;

        while i < stats.len() {
            if stats[i].0 as u8 == stat_name as u8 {
                return true;
            }
            i += 1;
        }
        false
    }

    pub const fn find_variants<const N: usize>(stat_name: StatName) -> [ItemId; N] {
        let mut i = 0;
        let mut j = 0;
        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
        while i < Self::VARIANTS {
            let item = Self::VALUES[i];
            if item.has_stat(stat_name) {
                result[j] = item;
                j += 1;
            }
            i += 1;
        }
        result
    }

    pub const fn filter(stat_name: StatName) -> &'static [Self] {
        Self::FILTERS[stat_name as usize]
    }

    pub const fn to_riot_id(&self) -> u32 {
        self.data().riot_id
    }

    pub const fn closure(&self) -> &'static [[Range<usize>; 2]; 2] {
        &Self::CLOSURES[self.index()]
    }

    pub const fn generator(&self) -> &'static Range<usize> {
        &ITEM_GENERATOR[self.index()]
    }

    pub const fn count_variants(stat_name: StatName) -> usize {
        let mut result = 0;
        let mut i = 0;
        while i < Self::VARIANTS {
            if Self::VALUES[i].has_stat(stat_name) {
                result += 1;
            }
            i += 1;
        }
        result
    }

    pub const fn deals_damage(&self) -> bool {
        let [mmin, _, rmin, _] = self.data().deals_damage;
        mmin || rmin
    }

    pub const fn deals_max_damage(&self) -> bool {
        let [_, mmax, _, rmax] = self.data().deals_damage;
        mmax || rmax
    }

    pub const fn price(&self) -> u16 {
        self.data().price
    }

    pub const fn identifiers(&self) -> &'static [CtxVar] {
        &self.data().identifiers
    }

    pub const fn metadata(&self) -> TypeMetadata<Self> {
        self.data().metadata
    }

    pub const fn eval(&self, ctx: &Ctx, attack_type: AttackType) -> [f32; 2] {
        item_const_eval(*self, ctx, attack_type)
    }

    pub const fn stats(&self) -> &'static [(StatName, u16)] {
        self.data().stats
    }
}

impl RuneId {
    pub const CLOSURES: &[[[Range<usize>; 2]; 2]; Self::VARIANTS] = &RUNE_CLOSURES;
    pub const GENERATORS: &[Range<usize>; Self::VARIANTS] = &RUNE_GENERATOR;
    pub const RIOT_IDS: [u32; Self::VARIANTS] = {
        let mut result = [0; _];
        let mut i = 0;
        while i < Self::VARIANTS {
            let value = Self::VALUES[i];
            result[i] = value.to_riot_id();
            i += 1;
        }
        result
    };

    pub const EXCEPTIONS: [Self; 14] = [
        Self::AbsorbLife,
        Self::Conqueror,
        Self::DeepWard,
        Self::EyeballCollection,
        Self::GhostPoro,
        Self::GrislyMementos,
        Self::GatheringStorm,
        Self::GraspOfTheUndying,
        Self::LethalTempo,
        Self::LegendAlacrity,
        Self::LegendBloodline,
        Self::LegendHaste,
        Self::ManaflowBand,
        Self::ZombieWard,
    ];

    pub const SIZE_OF_EXCEPTIONS: usize = bitset_size(bitset!(RuneId::EXCEPTIONS => [usize]));

    pub const fn exceptions() -> RunesExcSet {
        bitset!(RuneId::EXCEPTIONS)
    }

    pub const fn to_riot_id(&self) -> u32 {
        self.data().riot_id
    }

    pub const fn closure(&self) -> &'static [[Range<usize>; 2]; 2] {
        &Self::CLOSURES[self.index()]
    }

    pub const fn identifiers(&self) -> &'static [CtxVar] {
        &self.data().identifiers
    }

    pub const fn metadata(&self) -> TypeMetadata<Self> {
        self.data().metadata
    }

    pub const fn damage_type(&self) -> DamageType {
        self.data().metadata.damage_type
    }

    pub const fn eval(&self, ctx: &Ctx, attack_type: AttackType) -> [f32; 2] {
        rune_const_eval(*self, ctx, attack_type)
    }

    pub const fn deals_damage(&self) -> bool {
        let [mmin, _, rmin, _] = self.data().deals_damage;
        mmin || rmin
    }

    pub const fn deals_max_damage(&self) -> bool {
        let [_, mmax, _, rmax] = self.data().deals_damage;
        mmax || rmax
    }
}

macro_rules! impl_methods {
    (inner $stru:ident, $($repr:ty),*) => {
        pastey::paste! {
            $(
                impl TryFrom<$repr> for $stru {
                    type Error = &'static str;
                    fn try_from(value: $repr) -> Result<Self, Self::Error> {
                        Self::from_repr(value as _).ok_or(concat!(
                            "Index out of bounds when converting ",
                            stringify!($repr),
                            " to ",
                            stringify!($stru)
                        ))
                    }
                }

                impl TryFrom<&$repr> for $stru {
                    type Error = &'static str;
                    fn try_from(value: &$repr) -> Result<Self, Self::Error> {
                        Self::from_repr(*value as _).ok_or(concat!(
                            "Index out of bounds when converting ",
                            stringify!($repr),
                            " to ",
                            stringify!($stru)
                        ))
                    }
                }

                impl $stru {
                    pub const unsafe fn [<from_ $repr _unchecked>](id: $repr) -> Self {
                        unsafe { Self::from_repr_unchecked(id as _) }
                    }

                    pub const fn [<from_ $repr>](id: $repr) -> Option<Self> {
                        match id < Self::VARIANTS as _ {
                            true => unsafe { Some(Self::from_repr_unchecked(id as _)) },
                            false => None
                        }
                    }
                }
            )*
        }
    };
    ($($stru:ident => $repr:ty),+$(,)*) => {
        pastey::paste! {
            $(
                impl Default for $stru {
                    fn default() -> Self {
                        Self::default()
                    }
                }

                impl Display for $stru {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(f, "{}", self.name())
                    }
                }

                impl_methods!(inner $stru, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

                impl $stru {
                    pub const VALUES: [Self; Self::VARIANTS] = {
                        let mut i = 0;
                        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
                        while i < Self::VARIANTS {
                            result[i] = Self::from_repr(i as _).unwrap();
                            i += 1;
                        }
                        result
                    };

                    pub const NAMES: [&str; Self::VARIANTS] = {
                        let mut i = 0;
                        let mut result: [*const str; Self::VARIANTS] = unsafe { core::mem::zeroed() };
                        while i < Self::VARIANTS {
                            result[i] = Self::VALUES[i].name() as *const _;
                            i += 1;
                        }
                        unsafe { core::mem::transmute(result) }
                    };

                    pub const FORMULAS: &[Range<usize>; Self::VARIANTS] = &[<$stru:replace("Id", ""):upper _FORMULAS>];

                    pub const unsafe fn from_repr_unchecked(id: $repr) -> Self {
                        unsafe { core::mem::transmute(id) }
                    }

                    pub const fn from_repr(id: $repr) -> Option<Self> {
                        match id < Self::VARIANTS as _ {
                            true => unsafe { Some(Self::from_repr_unchecked(id as _)) },
                            false => None
                        }
                    }

                    pub const fn default() -> Self {
                        unsafe { Self::from_repr_unchecked(0) }
                    }

                    pub const fn data(&self) -> &'static [<$stru:replace("Id", "")>] {
                        [<$stru:replace("Id", "S"):upper _DATA>][self.index()]
                    }

                    pub const fn name(&self) -> &'static str {
                        self.data().name
                    }

                    pub const fn index(&self) -> usize {
                        *self as _
                    }
                }

                impl Sealed for $stru {}

                impl CastId for $stru {
                    const VARIANTS: usize = Self::VARIANTS;
                    const NAMES: &'static [&'static str] = &Self::NAMES;
                    const VALUES: &'static [Self] = &Self::VALUES;
                    const FORMULAS: &'static [Range<usize>] = Self::FORMULAS;
                    const GENERATORS: &'static [Range<usize>] = Self::GENERATORS;

                    fn entity(&self) -> EntityId {
                        EntityId::[<$stru:replace("Id", "")>](*self)
                    }

                    fn name(&self) -> &'static str {
                        self.name()
                    }

                    fn debug(&self) -> &'static str {
                        self.debug()
                    }

                    fn index(&self) -> usize {
                        self.index()
                    }
                }
            )+
        }
    };
}

impl_methods!(
    ChampionId => u8,
    ItemId => u16,
    RuneId => u8
);

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EntityId {
    Champion(ChampionId),
    Item(ItemId),
    Rune(RuneId),
}

impl<T: CastId> From<T> for EntityId {
    fn from(value: T) -> Self {
        value.entity()
    }
}

impl EntityId {
    pub const fn is_champion(&self) -> bool {
        matches!(self, EntityId::Champion(_))
    }

    pub const fn is_item(&self) -> bool {
        matches!(self, EntityId::Item(_))
    }

    pub const fn is_rune(&self) -> bool {
        matches!(self, EntityId::Rune(_))
    }
}

trait Sealed {}

#[allow(private_bounds)]
pub trait CastId
where
    Self: Any + Copy + Debug + Default + Sealed + Sized + 'static,
{
    const VARIANTS: usize;
    const NAMES: &'static [&'static str];
    const VALUES: &'static [Self];
    const FORMULAS: &'static [Range<usize>];
    const GENERATORS: &'static [Range<usize>];

    fn entity(&self) -> EntityId;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn debug(&self) -> &'static str;
    fn formula(&self) -> &'static Range<usize> {
        &Self::FORMULAS[self.index()]
    }
    fn generator(&self) -> &'static Range<usize> {
        &Self::GENERATORS[self.index()]
    }
    fn is_champion(&self) -> bool {
        self.entity().is_champion()
    }
    fn is_item(&self) -> bool {
        self.entity().is_item()
    }
    fn is_rune(&self) -> bool {
        self.entity().is_rune()
    }
}

pub trait ValueId: CastId {
    fn to_riot_id(&self) -> u32;
    fn identifiers(&self) -> &'static [CtxVar];
    fn functions(&self) -> &'static [[Range<usize>; 2]; 2];
    fn metadata(&self) -> TypeMetadata<Self>;
    fn damage_type(&self) -> DamageType {
        self.metadata().damage_type
    }
}

impl ValueId for ItemId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }

    fn identifiers(&self) -> &'static [CtxVar] {
        self.identifiers()
    }

    fn functions(&self) -> &'static [[Range<usize>; 2]; 2] {
        self.closure()
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}

impl ValueId for RuneId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }

    fn identifiers(&self) -> &'static [CtxVar] {
        self.identifiers()
    }

    fn functions(&self) -> &'static [[Range<usize>; 2]; 2] {
        self.closure()
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}
