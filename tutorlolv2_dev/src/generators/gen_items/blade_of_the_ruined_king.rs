use super::*;

impl Generator for BladeOfTheRuinedKing {
    fn generate(&mut self) -> MayFail {
        let [melee, ranged] = self.scaling(Passive(0))?;

        let damage = |scaling: f64| {
            let modifier = 1.minus(scaling).times(PhysicalMultiplier).parens();
            let numerator = scaling
                .times(EnemyCurrentHealth)
                .times(&modifier)
                .minus(AttackDamage)
                .plus(AttackDamage)
                .times(modifier)
                .parens();
            EnemyCurrentHealth.minus(numerator.div(scaling).parens())
        };

        self.damage_type(Physical)
            .melee_min_dmg(damage(melee))
            .ranged_min_dmg(damage(ranged))
            .end()
    }
}
