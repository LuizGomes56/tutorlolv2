#[cfg(feature = "engine")]
use {
    crate::items_code::item_const_eval,
    types::{AttackType, Ctx},
};

use {
    crate::{ItemId, bitset, bitset::*},
    core::mem::MaybeUninit,
    strum::{EnumCount, VariantArray},
    types::{DamageType, GameMap, MapMode, StatName, TypeMetadata},
};

impl ItemId {
    pub const NUMBER_OF_DAMAGING_ITEMS: usize = {
        let mut sum = 0;
        let mut i = 0;

        while i < Self::COUNT {
            let item = Self::from_repr(i as _).unwrap();
            if item.deals_damage() {
                sum += 1;
            }
            i += 1;
        }

        sum
    };

    pub const DAMAGING_IDS: [Self; Self::NUMBER_OF_DAMAGING_ITEMS] = {
        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        let mut j = 0;

        while i < Self::COUNT {
            let item = Self::from_repr(i as _).unwrap();

            if item.deals_damage() {
                result[j] = item;
                j += 1;
            }

            i += 1;
        }

        result
    };

    pub const DAMAGING: BitSet = bitset!(ItemId::DAMAGING_IDS);

    pub const RIOT_IDS: [u32; Self::COUNT] = {
        let mut result = [0; _];
        let mut i = 0;

        while i < Self::COUNT {
            let value = Self::VARIANTS[i];
            result[i] = value.riot_id();
            i += 1;
        }

        result
    };

    pub const SIML_METADATA: [TypeMetadata<Self>; ItemId::L_SIML] = {
        let mut siml_items = MaybeUninit::<[TypeMetadata<Self>; ItemId::L_SIML]>::uninit();
        let siml_items_ptr = siml_items.as_mut_ptr();
        let mut i = 0;

        while i < ItemId::L_SIML {
            let item_id = Self::SIML[i];
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

    pub const L_SIML: usize = {
        let mut sum = 0;
        let mut i = 0;

        while i < Self::COUNT {
            if Self::DATA[i].is_siml() {
                sum += 1;
            }
            i += 1;
        }

        sum
    };

    pub const SIML: [Self; Self::L_SIML] = {
        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        let mut j = 0;

        while i < Self::COUNT {
            if Self::DATA[i].is_siml() {
                result[j] = Self::from_repr(i as _).unwrap();
                j += 1;
            }
            i += 1;
        }
        result
    };

    pub const fn is_siml(&self) -> bool {
        let mut i = 0;

        while i < Self::L_SIML {
            if self.index() == Self::SIML[i].index() {
                return true;
            }
            i += 1;
        }

        false
    }

    pub const fn indexof_siml(index: usize) -> Option<Self> {
        let mut i = 0;
        while i < Self::L_SIML {
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

        while i < Self::COUNT {
            let item = Self::VARIANTS[i];
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

    pub const fn riot_id(&self) -> u32 {
        self.data().riot_id
    }

    pub const fn with_stat(stat_name: StatName) -> usize {
        let mut result = 0;
        let mut i = 0;

        while i < Self::COUNT {
            if Self::VARIANTS[i].has_stat(stat_name) {
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

    pub const fn metadata(&self) -> TypeMetadata<Self> {
        self.data().metadata
    }

    #[cfg(feature = "engine")]
    pub const fn eval(&self, ctx: &Ctx, attack_type: AttackType) -> [f32; 2] {
        item_const_eval(*self, ctx, attack_type)
    }

    pub const fn stats(&self) -> &'static [(StatName, u16)] {
        self.data().stats
    }

    /// Stats of this item on `game_map`. Summoner's Rift changes none, and nearly
    /// every game is played there, so the search is cold
    #[inline(always)]
    pub const fn stats_on(&self, game_map: GameMap) -> &'static [(StatName, u16)] {
        let mode = game_map.mode();

        if mode as u8 == MapMode::Default as u8 {
            return self.stats();
        }

        self.find_map_stats(mode)
    }

    /// Search of [`Self::stats_on`], kept out of line so that the check of
    /// Summoner's Rift inlines into the loops over items
    #[cold]
    #[inline(never)]
    const fn find_map_stats(&self, mode: MapMode) -> &'static [(StatName, u16)] {
        let list = self.data().map_stats;
        let mut i = 0;

        while i < list.len() {
            if list[i].0 as u8 == mode as u8 {
                return list[i].1;
            }

            i += 1;
        }

        self.stats()
    }

    pub const fn price(&self) -> u16 {
        self.data().price
    }

    pub const fn tier(&self) -> u8 {
        self.data().tier
    }

    pub const fn purchasable(&self) -> bool {
        self.data().purchasable
    }

    pub const fn maps(&self) -> &'static [GameMap] {
        self.data().maps
    }

    #[cfg(any(feature = "yew", feature = "formula-docs"))]
    pub const fn builds_from(&self) -> &'static [Self] {
        self.data().builds_from
    }

    #[cfg(any(feature = "yew", feature = "formula-docs"))]
    pub const fn builds_into(&self) -> &'static [Self] {
        self.data().builds_into
    }

    pub const fn has_map(&self, game_map: GameMap) -> bool {
        let maps = self.maps();
        let mut i = 0;

        while i < maps.len() {
            if maps[i] as u8 == game_map as u8 {
                return true;
            }
            i += 1;
        }

        false
    }

    #[cfg(feature = "client-data")]
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

    #[cfg(feature = "client-data")]
    pub const ENEMY_EXCEPTIONS: [Self; 3] =
        [Self::Dragonheart, Self::DemonKingsCrown, Self::BlackCleaver];

    #[cfg(feature = "client-data")]
    pub const SIZE_OF_EXCEPTIONS: usize = max_usize(
        bitset_size(bitset!(ItemId::ALLY_EXCEPTIONS => [usize])),
        bitset_size(bitset!(ItemId::ENEMY_EXCEPTIONS => [usize])),
    );

    #[cfg(feature = "client-data")]
    pub const fn exceptions(ally: bool) -> BitSetExc {
        match ally {
            true => bitset!(ItemId::ALLY_EXCEPTIONS),
            false => bitset!(ItemId::ENEMY_EXCEPTIONS),
        }
    }

    #[cfg(feature = "client-data")]
    pub const fn identifiers(&self) -> &'static [types::CtxVar] {
        self.data().identifiers
    }
}

macro_rules! impl_item_filters {
    ($($name:ident),+$(,)?) => {
        impl ItemId {
            pastey::paste! {
                pub const FILTERS: [&[Self]; StatName::COUNT] = [
                    $(
                        &Self::[<ITEMS_WITH_ $name:snake:upper>],
                    )+
                ];

                $(
                    pub const [<ITEMS_WITH_ $name:snake:upper>]: [Self; Self::with_stat(StatName::$name)] =
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
