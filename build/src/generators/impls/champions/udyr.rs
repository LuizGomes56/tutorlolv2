use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min),  /* Bonus Physical Damage */
                (2, Void), /* Bonus Physical Damage On-Hit */
                (3, Max),  /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (1, Min), /* Magic Damage per Tick */
                (3, Max), /* Total Magic Damage */
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
