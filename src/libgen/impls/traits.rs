use {
    crate::{Champion, ChampionId, EntityId, Item, ItemId, Rune, RuneId},
    core::{
        any::Any,
        fmt::{Debug, Display},
        str::FromStr,
    },
    tutorlolv2_types::{DamageType, TypeMetadata},
};

#[cfg(feature = "docs")]
use {core::ops::Range, tutorlolv2_types::CtxVar};

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

                impl FromStr for $stru {
                    type Err = &'static str;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        $crate::[<$stru:replace("Id", "s"):lower _code>]
                        ::[<$stru:replace("Id", ""):upper _NAME_TO_ID>]
                            .get(s)
                            .copied()
                            .ok_or(concat!("No matches when calling ", stringify!($stru), "::from_str"))
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

                    pub const NAME_TO_ID: &phf::Map<&str, Self> =
                        &$crate::[<$stru:replace("Id", "s"):lower _code>]
                        ::[<$stru:replace("Id", ""):upper _NAME_TO_ID>];

                    #[cfg(feature = "docs")]
                    pub const GENERATOR_DOCS: &[Range<usize>; Self::VARIANTS] =
                        &$crate::docs::[<$stru:replace("Id", ""):upper _GENERATORS>];

                    #[cfg(feature = "docs")]
                    pub const DOCS: &[Range<usize>; Self::VARIANTS] =
                        &$crate::docs::[<$stru:replace("Id", ""):upper _FORMULAS>];

                    pub const DATA: &[&[<$stru:replace("Id", "")>]] =
                        &$crate::[<$stru:replace("Id", "s"):lower _code>]
                        ::[<$stru:replace("Id", "S"):upper _DATA>];

                    pub const unsafe fn from_repr_unchecked(id: $repr) -> Self {
                        unsafe { core::mem::transmute(id) }
                    }

                    pub const fn from_repr(id: $repr) -> Option<Self> {
                        match id < Self::VARIANTS as _ {
                            true => unsafe { Some(Self::from_repr_unchecked(id as _)) },
                            false => None
                        }
                    }

                    pub const fn generator_docs(&self) -> &'static Range<usize> {
                        &Self::GENERATOR_DOCS[self.index()]
                    }

                    pub const fn docs(&self) -> &'static Range<usize> {
                        &Self::DOCS[self.index()]
                    }

                    pub const fn default() -> Self {
                        unsafe { Self::from_repr_unchecked(0) }
                    }

                    pub const fn data(&self) -> &'static [<$stru:replace("Id", "")>] {
                        Self::DATA[self.index()]
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

                    #[cfg(feature = "docs")]
                    const GENERATOR_DOCS: &'static [Range<usize>] = Self::GENERATOR_DOCS;

                    #[cfg(feature = "docs")]
                    const DOCS: &'static [Range<usize>] = Self::DOCS;

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

trait Sealed {}

#[allow(private_bounds)]
pub trait CastId
where
    Self: Any + Copy + Debug + Default + Sealed + Sized + 'static,
{
    const VARIANTS: usize;
    const NAMES: &'static [&'static str];
    const VALUES: &'static [Self];

    #[cfg(feature = "docs")]
    const GENERATOR_DOCS: &'static [Range<usize>];

    #[cfg(feature = "docs")]
    const DOCS: &'static [Range<usize>];

    fn entity(&self) -> EntityId;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn debug(&self) -> &'static str;

    #[cfg(feature = "docs")]
    fn docs(&self) -> &'static Range<usize> {
        &Self::DOCS[self.index()]
    }

    #[cfg(feature = "docs")]
    fn generator_docs(&self) -> &'static Range<usize> {
        &Self::GENERATOR_DOCS[self.index()]
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
    fn metadata(&self) -> TypeMetadata<Self>;

    #[cfg(feature = "docs")]
    fn identifiers(&self) -> &'static [CtxVar];

    #[cfg(feature = "docs")]
    fn functions_docs(&self) -> &'static [[Range<usize>; 2]; 2];

    fn damage_type(&self) -> DamageType {
        self.metadata().damage_type
    }
}

impl ValueId for ItemId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }

    #[cfg(feature = "docs")]
    fn identifiers(&self) -> &'static [CtxVar] {
        self.identifiers()
    }

    #[cfg(feature = "docs")]
    fn functions_docs(&self) -> &'static [[Range<usize>; 2]; 2] {
        self.functions_docs()
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}

impl ValueId for RuneId {
    fn to_riot_id(&self) -> u32 {
        self.to_riot_id()
    }

    #[cfg(feature = "docs")]
    fn identifiers(&self) -> &'static [CtxVar] {
        self.identifiers()
    }

    #[cfg(feature = "docs")]
    fn functions_docs(&self) -> &'static [[Range<usize>; 2]; 2] {
        self.functions_docs()
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}
