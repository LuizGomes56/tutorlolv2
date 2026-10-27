use super::*;

impl Generator for BladeOfTheRuinedKing {
    fn generate(&mut self) -> MayFail {
        let ratio = |attack_type| -> MayFail<f64> {
            self.effect_for(Passive, attack_type)?.scalings[0]
                .try_get_value()
                .ok_or(format!("Unknown {attack_type:?} percent scaling").into())
        };

        let melee_ratio = ratio(Melee)?;
        let ranged_ratio = ratio(Ranged)?;

        let damage = |scaling: &dyn Display| {
            let modifier = f![(1 - scaling * PhysicalMultiplier)];
            f![EnemyCurrentHealth
                - ((scaling * EnemyCurrentHealth * modifier - AttackDamage
                    + AttackDamage * modifier)
                    / scaling)]
        };

        self.damage_type(Physical)
            .assign(Melee, Min, damage(&melee_ratio))
            .assign(Ranged, Min, damage(&ranged_ratio))
            .end()
    }
}
