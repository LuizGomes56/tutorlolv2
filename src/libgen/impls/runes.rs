pub use {
    crate::{ChampionId, ItemId, RuneId, bitset, bitset::*, runes_code::rune_const_eval},
    bincode::{Decode, Encode},
    core::{
        any::Any,
        fmt::{Debug, Display},
        mem::MaybeUninit,
        str::FromStr,
    },
    serde::{Deserialize, Serialize},
    tutorlolv2_types::{
        AbilityId::{self, *},
        AbilityName::{self, *},
        AdaptiveType,
        AttackType::{self, *},
        Attrs::*,
        ComboElement::{self, *},
        Ctx,
        CtxVar::*,
        DamageType::{self, *},
        GameMap::{self, *},
        Key, MergeData,
        Position::{self, *},
        StatName, TypeMetadata,
    },
};

impl RuneId {
    pub const NUMBER_OF_DAMAGING_RUNES: usize = {
        let mut sum = 0;
        let mut i = 0;
        while i < Self::VARIANTS {
            let rune = Self::DATA[i];
            let [mmin, mmax, rmin, rmax] = rune.deals_damage;
            if mmin || mmax || rmin || rmax {
                sum += 1;
            }
            i += 1;
        }
        sum
    };

    pub const DAMAGING_RUNES_ARRAY: [Self; Self::NUMBER_OF_DAMAGING_RUNES] = {
        let mut result: [Self; _] = unsafe { core::mem::zeroed() };
        let mut i = 0;
        let mut j = 0;

        while i < Self::VARIANTS {
            let rune = Self::DATA[i];
            let [mmin, mmax, rmin, rmax] = rune.deals_damage;

            if mmin || mmax || rmin || rmax {
                result[j] = Self::from_repr(i as _).unwrap();
                j += 1;
            }

            i += 1;
        }
        result
    };

    pub const DAMAGING_RUNES: RunesBitSet = bitset!(RuneId::DAMAGING_RUNES_ARRAY);

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

    pub const fn metadata(&self) -> TypeMetadata<Self> {
        self.data().metadata
    }

    pub const fn damage_type(&self) -> DamageType {
        self.metadata().damage_type
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
