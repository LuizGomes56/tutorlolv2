use {
    crate::{
        Champion, ChampionId, EntityId, Item, ItemId, Rune, RuneId, bitset::BitSet,
        model::ValueException,
    },
    core::{
        any::Any,
        fmt::{Debug, Display},
        str::FromStr,
    },
    tutorlolv2_types::{DamageType, Position, TypeMetadata},
};

macro_rules! impl_methods {
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
                            .ok_or(concat!(
                                "No matches when calling ",
                                stringify!($stru),
                                "::from_str"
                            ))
                    }
                }

                impl $stru {
                    pub const NAME_TO_ID: &phf::Map<&str, Self> =
                        &$crate::[<$stru:replace("Id", "s"):lower _code>]
                        ::[<$stru:replace("Id", ""):upper _NAME_TO_ID>];

                    pub const DATA: &[&[<$stru:replace("Id", "")>]] =
                        &$crate::[<$stru:replace("Id", "s"):lower _code>]
                        ::[<$stru:replace("Id", "S"):upper _DATA>];

                    pub const fn default() -> Self {
                        Self::from_repr(0).unwrap()
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
                    fn entity(&self) -> EntityId {
                        EntityId::[<$stru:replace("Id", "")>](*self)
                    }

                    fn name(&self) -> &'static str {
                        self.name()
                    }

                    fn debug(&self) -> &'static str {
                        <Self as strum::VariantNames>::VARIANTS[self.index()]
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
    fn entity(&self) -> EntityId;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn debug(&self) -> &'static str;

    #[cfg(feature = "yew")]
    fn render_generator(&self) -> Option<alloc::string::String> {
        crate::yew::render::GENERATOR_RENDERER.render(*self)
    }

    #[cfg(feature = "yew")]
    fn render_global(&self) -> crate::yew::render::MayFail<alloc::string::String> {
        match self.entity() {
            EntityId::Champion(v) => v.render_global(),
            EntityId::Item(v) => v.render_global(),
            EntityId::Rune(v) => v.render_global(),
        }
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
    const DAMAGING: &BitSet;
    const DAMAGING_IDS: &[Self];

    fn riot_id(&self) -> u32;
    fn metadata(&self) -> TypeMetadata<Self>;
    fn pack_exc(&self, v: u32) -> ValueException;
    fn recommendations(champion_id: ChampionId, position: Position) -> &'static [Self];

    #[cfg(feature = "yew")]
    fn exceptions(ally: bool) -> crate::bitset::BitSetExc;

    #[cfg(feature = "yew")]
    fn identifiers(&self) -> &'static [tutorlolv2_types::CtxVar];

    #[cfg(feature = "yew")]
    fn render_fn(&self) -> crate::yew::render::MayFail<alloc::string::String>;

    fn damage_type(&self) -> DamageType {
        self.metadata().damage_type
    }
}

impl ValueId for ItemId {
    const DAMAGING: &BitSet = &Self::DAMAGING;
    const DAMAGING_IDS: &[Self] = &Self::DAMAGING_IDS;

    #[cfg(feature = "yew")]
    fn render_fn(&self) -> crate::yew::render::MayFail<alloc::string::String> {
        self.render_fn()
    }

    #[cfg(feature = "yew")]
    fn exceptions(ally: bool) -> BitSetExc {
        ItemId::exceptions(ally)
    }

    #[cfg(feature = "yew")]
    fn identifiers(&self) -> &'static [tutorlolv2_types::CtxVar] {
        self.identifiers()
    }

    fn riot_id(&self) -> u32 {
        self.riot_id()
    }

    fn pack_exc(&self, v: u32) -> ValueException {
        ValueException::pack_item_id(*self, v)
    }

    fn recommendations(champion_id: ChampionId, position: Position) -> &'static [Self] {
        champion_id.recommended_items(position)
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}

impl ValueId for RuneId {
    const DAMAGING: &BitSet = &Self::DAMAGING;
    const DAMAGING_IDS: &[Self] = &Self::DAMAGING_IDS;

    #[cfg(feature = "yew")]
    fn render_fn(&self) -> crate::yew::render::MayFail<alloc::string::String> {
        self.render_fn()
    }

    #[cfg(feature = "yew")]
    fn exceptions(_: bool) -> BitSetExc {
        RuneId::exceptions()
    }

    #[cfg(feature = "yew")]
    fn identifiers(&self) -> &'static [tutorlolv2_types::CtxVar] {
        self.identifiers()
    }

    fn riot_id(&self) -> u32 {
        self.riot_id()
    }

    fn pack_exc(&self, v: u32) -> ValueException {
        ValueException::pack_rune_id(*self, v)
    }

    fn recommendations(champion_id: ChampionId, position: Position) -> &'static [Self] {
        champion_id.recommended_runes(position)
    }

    fn metadata(&self) -> TypeMetadata<Self> {
        self.metadata()
    }
}
