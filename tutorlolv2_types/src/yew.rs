//! Integration with `yew`: reducers and CSS classes of shared types.
use crate::{AbilityLevels, AdaptiveType, DamageType};
use alloc::rc::Rc;
use yew::{Classes, Reducible};

/// Applies an action to a value in place, so reducers can share the logic
pub trait ReduceApply
where
    Self: Copy + PartialEq + 'static,
    Self::Action: PartialEq + Copy,
{
    type Action;

    fn apply(&mut self, action: Self::Action);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbilityLevelsAction {
    Q(u8),
    W(u8),
    E(u8),
    R(u8),
}

impl AbilityLevelsAction {
    /// Actions in the order of [`AbilityLevels::ABILITIES`]
    pub const ALL: [fn(u8) -> Self; 4] = [Self::Q, Self::W, Self::E, Self::R];
}

impl ReduceApply for AbilityLevels {
    type Action = AbilityLevelsAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Q(value) => self.q = value,
            Self::Action::W(value) => self.w = value,
            Self::Action::E(value) => self.e = value,
            Self::Action::R(value) => self.r = value,
        }
    }
}

impl Reducible for AbilityLevels {
    type Action = AbilityLevelsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = self;
        let new_mut = Rc::make_mut(&mut new);

        new_mut.apply(action);
        new
    }
}

impl DamageType {
    /// Text color class of this damage type
    pub const fn class(&self) -> &'static str {
        match self {
            Self::Physical => "text-orange-500",
            Self::Magic => "text-sky-500",
            Self::Mixed => "text-indigo-500",
            Self::True => "text-white",
            Self::Adaptive => "text-purple-500",
            Self::Unspecified => "text-std-300",
        }
    }
}

impl AdaptiveType {
    /// Text color class of the damage type this adaptive type resolves to
    pub const fn class(&self) -> &'static str {
        match self {
            Self::Magic => DamageType::Magic.class(),
            Self::Physical => DamageType::Physical.class(),
        }
    }
}

impl From<DamageType> for Classes {
    fn from(value: DamageType) -> Self {
        Classes::from(value.class())
    }
}

impl From<AdaptiveType> for Classes {
    fn from(value: AdaptiveType) -> Self {
        Classes::from(value.class())
    }
}
