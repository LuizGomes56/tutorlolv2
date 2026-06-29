use {
    crate::{model::Dragons, yew::ReduceApply},
    alloc::rc::Rc,
    yew::Reducible,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DragonsAction {
    AllyFire(u16),
    AllyEarth(u16),
    AllyChemtech(u16),
    EnemyEarth(u16),
}

impl ReduceApply for Dragons {
    type Action = DragonsAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::AllyFire(value) => self.ally_fire_dragons = value,
            Self::Action::AllyEarth(value) => self.ally_earth_dragons = value,
            Self::Action::AllyChemtech(value) => self.ally_chemtech_dragons = value,
            Self::Action::EnemyEarth(value) => self.enemy_earth_dragons = value,
        }
    }
}

impl Reducible for Dragons {
    type Action = DragonsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = *self;
        <Self as ReduceApply>::apply(&mut new, action);
        Rc::new(new)
    }
}
