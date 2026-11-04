use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Bonus Physical Damage", Min),
                ("Bonus Physical Damage On-Hit", Void),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::R,
            [
                ("Magic Damage per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .modify(Q(Min), |_| {
            f![(0.03 + 0.01 * (QLevel - 1) + 0.00035 * BonusAd) * EnemyMaxHealth]
        })?
        .modify(Q(Max), |_| {
            f![(0.06 + 0.02 * (QLevel - 1) + 0.0007 * BonusAd) * EnemyMaxHealth]
        })?
        .damage_types([Q(Void), Q(Min), Q(Max)], Physical)?
        .end()
    }
}
