use super::*;

impl Generator for BladeOfTheRuinedKing {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        let melee_ratio = self.effect(Passive)?.scalings[0]
            .try_get_value()
            .ok_or("Unknown melee percent scaling")?;

        let ranged_ratio = melee_ratio.div(1.5);

        let damage = |scaling: &dyn Display| {
            format!(
                "{EnemyCurrentHealth} - ({numerator} / {scaling})",
                numerator = format_args!(
                    "({scaling} * {EnemyCurrentHealth} * {modifier} - {AttackDamage} + {AttackDamage} * {modifier})",
                    modifier = format_args!("(1 - {scaling} * {PhysicalMultiplier})"),
                ),
            )
        };

        self.damage_type(Physical)
            .assign(Melee, Min, &damage(&melee_ratio))
            .assign(Ranged, Min, &damage(&ranged_ratio))
            .end()
    }
}
