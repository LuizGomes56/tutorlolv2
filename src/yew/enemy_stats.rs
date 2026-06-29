use {
    crate::{
        model::EnemyStats,
        yew::{ReduceApply, stats::PlayerStatsField},
    },
    alloc::rc::Rc,
    core::ops::{Index, IndexMut},
    yew::Reducible,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnemyStatsAction {
    Armor(i32),
    CurrentHealth(i32),
    MagicResist(i32),
    MaxHealth(i32),
    MissingHealth(i32),
}

impl ReduceApply for EnemyStats {
    type Action = EnemyStatsAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Armor(value) => self.armor = value as _,
            Self::Action::CurrentHealth(value) => self.current_health = value as _,
            Self::Action::MagicResist(value) => self.magic_resist = value as _,
            Self::Action::MaxHealth(value) => self.max_health = value as _,
            Self::Action::MissingHealth(value) => self.missing_health = value as _,
        }
    }
}

impl Reducible for EnemyStats {
    type Action = EnemyStatsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = *self;
        <Self as ReduceApply>::apply(&mut new, action);
        Rc::new(new)
    }
}

impl Index<PlayerStatsField> for EnemyStats {
    type Output = f32;

    fn index(&self, index: PlayerStatsField) -> &Self::Output {
        match index {
            PlayerStatsField::Armor => &self.armor,
            PlayerStatsField::CurrentHealth => &self.current_health,
            PlayerStatsField::MagicResist => &self.magic_resist,
            PlayerStatsField::MaxHealth => &self.max_health,
            _ => panic!("Can't use PlayerStatsField::{index:?} to index into EnemyStats"),
        }
    }
}

impl IndexMut<PlayerStatsField> for EnemyStats {
    fn index_mut(&mut self, index: PlayerStatsField) -> &mut Self::Output {
        match index {
            PlayerStatsField::Armor => &mut self.armor,
            PlayerStatsField::CurrentHealth => &mut self.current_health,
            PlayerStatsField::MagicResist => &mut self.magic_resist,
            PlayerStatsField::MaxHealth => &mut self.max_health,
            _ => panic!("Can't use PlayerStatsField::{index:?} to index into EnemyStats"),
        }
    }
}
