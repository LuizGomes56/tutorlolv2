use super::*;

impl Generator for BladeOfTheRuinedKing {
    fn generate(&mut self) -> MayFail {
        let ratio = |attack_type| -> MayFail<Formula> {
            Ok(self
                .formula_for(Named("Mist's Edge"), attack_type)?
                .replace_var(EnemyCurrentHealth, &Formula::Const(1.0)))
        };

        let melee_ratio = ratio(Melee)?;
        let ranged_ratio = ratio(Ranged)?;

        let damage = |scaling: Formula| {
            let modifier = f![(1 - scaling * PhysicalMultiplier)];
            f![EnemyCurrentHealth
                - ((scaling * EnemyCurrentHealth * modifier - AttackDamage
                    + AttackDamage * modifier)
                    / scaling)]
        };

        self.damage_type(Physical)
            .assign(Melee, Min, damage(melee_ratio))
            .assign(Ranged, Min, damage(ranged_ratio))
            .end()
    }
}
