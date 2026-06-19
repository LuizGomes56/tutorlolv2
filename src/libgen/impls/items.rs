use {
    crate::{ItemId, bitset, bitset::*, items_code::item_const_eval},
    core::mem::MaybeUninit,
    tutorlolv2_types::{AttackType, Ctx, DamageType, GameMap, StatName, TypeMetadata},
};

impl ItemId {
    pub const NUMBER_OF_DAMAGING_ITEMS: usize = {
        let mut sum = 0;
        let mut i = 0;

        while i < Self::VARIANTS {
            let item = Self::from_repr(i as _).unwrap();
            if item.deals_damage() {
                sum += 1;
            }
            i += 1;
        }

        sum
    };

    pub const DAMAGING_ITEMS_ARRAY: [Self; Self::NUMBER_OF_DAMAGING_ITEMS] = {
        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        let mut j = 0;

        while i < Self::VARIANTS {
            let item = Self::from_repr(i as _).unwrap();

            if item.deals_damage() {
                result[j] = item;
                j += 1;
            }

            i += 1;
        }

        result
    };

    pub const DAMAGING_ITEMS: ItemsBitSet = bitset!(ItemId::DAMAGING_ITEMS_ARRAY);

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

        while i < Self::VARIANTS {
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

        while i < Self::VARIANTS {
            if Self::DATA[i].is_siml() {
                result[j] = Self::from_repr(i as _).unwrap();
                j += 1;
            }
            i += 1;
        }
        result
    };

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
