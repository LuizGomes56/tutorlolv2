use {
    crate::{model::AbilityLevels, yew::ReduceApply},
    alloc::rc::Rc,
    core::ops::{Index, IndexMut},
    tutorlolv2_types::Key,
    yew::Reducible,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbilityLevelsAction {
    Q(u8),
    W(u8),
    E(u8),
    R(u8),
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
        let mut new = *self;
        <Self as ReduceApply>::apply(&mut new, action);
        Rc::new(new)
    }
}

impl AbilityLevels {
    pub const ABILITIES: [Key; 4] = [Key::Q, Key::W, Key::E, Key::R];

    pub const ACTIONS: [fn(u8) -> AbilityLevelsAction; 4] = [
        AbilityLevelsAction::Q,
        AbilityLevelsAction::W,
        AbilityLevelsAction::E,
        AbilityLevelsAction::R,
    ];
}

impl Index<Key> for AbilityLevels {
    type Output = u8;

    fn index(&self, index: Key) -> &Self::Output {
        match index {
            Key::P => panic!("Can't use Key::P to index AbilityLevels"),
            Key::Q => &self.q,
            Key::W => &self.w,
            Key::E => &self.e,
            Key::R => &self.r,
        }
    }
}

impl IndexMut<Key> for AbilityLevels {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        match index {
            Key::P => panic!("Can't use Key::P to index AbilityLevels"),
            Key::Q => &mut self.q,
            Key::W => &mut self.w,
            Key::E => &mut self.e,
            Key::R => &mut self.r,
        }
    }
}
