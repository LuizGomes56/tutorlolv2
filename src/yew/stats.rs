use {
    crate::{model::PlayerStats, yew::ReduceApply},
    alloc::rc::Rc,
    core::{
        fmt::Display,
        ops::{Index, IndexMut},
    },
    yew::Reducible,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerStatsAction {
    AbilityPower(i32),
    Armor(i32),
    ArmorPenetrationFlat(i32),
    ArmorPenetrationPercent(i32),
    AttackDamage(i32),
    AttackSpeed(i32),
    CritChance(i32),
    CritDamage(i32),
    CurrentHealth(i32),
    MagicPenetrationFlat(i32),
    MagicPenetrationPercent(i32),
    MagicResist(i32),
    MaxHealth(i32),
    MaxMana(i32),
    CurrentMana(i32),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlayerStatsField {
    AbilityPower,
    Armor,
    ArmorPenetrationFlat,
    ArmorPenetrationPercent,
    AttackDamage,
    AttackSpeed,
    CritChance,
    CritDamage,
    CurrentHealth,
    MagicPenetrationFlat,
    MagicPenetrationPercent,
    MagicResist,
    MaxHealth,
    MaxMana,
    CurrentMana,
}

impl Display for PlayerStatsField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            PlayerStatsField::AbilityPower => "Ability Power",
            PlayerStatsField::Armor => "Armor",
            PlayerStatsField::ArmorPenetrationFlat => "Armor Pen. Flat",
            PlayerStatsField::ArmorPenetrationPercent => "Armor Pen. %",
            PlayerStatsField::AttackDamage => "Attack Damage",
            PlayerStatsField::AttackSpeed => "Attack Speed",
            PlayerStatsField::CritChance => "Crit Chance",
            PlayerStatsField::CritDamage => "Crit Damage",
            PlayerStatsField::CurrentHealth => "Current Health",
            PlayerStatsField::MagicPenetrationFlat => "Magic Pen. Flat",
            PlayerStatsField::MagicPenetrationPercent => "Magic Pen. %",
            PlayerStatsField::MagicResist => "Magic Resist",
            PlayerStatsField::MaxHealth => "Max Health",
            PlayerStatsField::MaxMana => "Max Mana",
            PlayerStatsField::CurrentMana => "Current Mana",
        };

        write!(f, "{name}")
    }
}

impl ReduceApply for PlayerStats {
    type Action = PlayerStatsAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::AbilityPower(value) => self.ability_power = value as _,
            Self::Action::Armor(value) => self.armor = value as _,
            Self::Action::ArmorPenetrationFlat(value) => self.armor_penetration_flat = value as _,
            Self::Action::ArmorPenetrationPercent(value) => {
                self.armor_penetration_percent = value as _
            }
            Self::Action::AttackDamage(value) => self.attack_damage = value as _,
            Self::Action::AttackSpeed(value) => self.attack_speed = value as _,
            Self::Action::CritChance(value) => self.crit_chance = value as _,
            Self::Action::CritDamage(value) => self.crit_damage = value as _,
            Self::Action::CurrentHealth(value) => self.current_health = value as _,
            Self::Action::MagicPenetrationFlat(value) => self.magic_penetration_flat = value as _,
            Self::Action::MagicPenetrationPercent(value) => {
                self.magic_penetration_percent = value as _
            }
            Self::Action::MagicResist(value) => self.magic_resist = value as _,
            Self::Action::MaxHealth(value) => self.max_health = value as _,
            Self::Action::MaxMana(value) => self.max_mana = value as _,
            Self::Action::CurrentMana(value) => self.current_mana = value as _,
        }
    }
}

impl Reducible for PlayerStats {
    type Action = PlayerStatsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = *self;
        <Self as ReduceApply>::apply(&mut new, action);
        Rc::new(new)
    }
}

impl Index<PlayerStatsField> for PlayerStats {
    type Output = f32;

    fn index(&self, index: PlayerStatsField) -> &Self::Output {
        match index {
            PlayerStatsField::AbilityPower => &self.ability_power,
            PlayerStatsField::Armor => &self.armor,
            PlayerStatsField::ArmorPenetrationFlat => &self.armor_penetration_flat,
            PlayerStatsField::ArmorPenetrationPercent => &self.armor_penetration_percent,
            PlayerStatsField::AttackDamage => &self.attack_damage,
            PlayerStatsField::AttackSpeed => &self.attack_speed,
            PlayerStatsField::CritChance => &self.crit_chance,
            PlayerStatsField::CritDamage => &self.crit_damage,
            PlayerStatsField::CurrentHealth => &self.current_health,
            PlayerStatsField::MagicPenetrationFlat => &self.magic_penetration_flat,
            PlayerStatsField::MagicPenetrationPercent => &self.magic_penetration_percent,
            PlayerStatsField::MagicResist => &self.magic_resist,
            PlayerStatsField::MaxHealth => &self.max_health,
            PlayerStatsField::MaxMana => &self.max_mana,
            PlayerStatsField::CurrentMana => &self.current_mana,
        }
    }
}

impl IndexMut<PlayerStatsField> for PlayerStats {
    fn index_mut(&mut self, index: PlayerStatsField) -> &mut Self::Output {
        match index {
            PlayerStatsField::AbilityPower => &mut self.ability_power,
            PlayerStatsField::Armor => &mut self.armor,
            PlayerStatsField::ArmorPenetrationFlat => &mut self.armor_penetration_flat,
            PlayerStatsField::ArmorPenetrationPercent => &mut self.armor_penetration_percent,
            PlayerStatsField::AttackDamage => &mut self.attack_damage,
            PlayerStatsField::AttackSpeed => &mut self.attack_speed,
            PlayerStatsField::CritChance => &mut self.crit_chance,
            PlayerStatsField::CritDamage => &mut self.crit_damage,
            PlayerStatsField::CurrentHealth => &mut self.current_health,
            PlayerStatsField::MagicPenetrationFlat => &mut self.magic_penetration_flat,
            PlayerStatsField::MagicPenetrationPercent => &mut self.magic_penetration_percent,
            PlayerStatsField::MagicResist => &mut self.magic_resist,
            PlayerStatsField::MaxHealth => &mut self.max_health,
            PlayerStatsField::MaxMana => &mut self.max_mana,
            PlayerStatsField::CurrentMana => &mut self.current_mana,
        }
    }
}
