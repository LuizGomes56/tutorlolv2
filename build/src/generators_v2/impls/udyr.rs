use super::*;

impl GeneratorV2 for Udyr {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Min))?
            .take("Bonus Physical Damage On-Hit", Q(Void))?
            .take("Total Physical Damage", Q(Max))?
            .take("Magic Damage per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .set(Q(Min), {
                f![(0.03 + 0.01 * (QLevel - 1) + 0.00035 * BonusAd) * EnemyMaxHealth]
            })
            .set(Q(Max), {
                f![(0.06 + 0.02 * (QLevel - 1) + 0.0007 * BonusAd) * EnemyMaxHealth]
            })
            .damage_type(Q(Void), Physical)
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .end()
    }
}
