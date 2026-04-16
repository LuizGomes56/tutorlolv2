use super::*;

impl Generator<ItemData> for BladeOfTheRuinedKing {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = self.scalings(Passive(0))?;

        let damage = |scaling| {
            format!(
                "{EnemyCurrentHealth} - ({numerator} / {scaling})",
                numerator = format_args!(
                    "({scaling} * {EnemyCurrentHealth} * {modifier} - {AttackDamage} + {AttackDamage} * {modifier})",
                    modifier = format_args!("(1 - {scaling} * {PhysicalMultiplier})"),
                ),
            )
        };

        self.damage_type(Physical);
        self.melee_min_dmg(damage(melee));
        self.ranged_min_dmg(damage(ranged));
        self.end()
    }
}
