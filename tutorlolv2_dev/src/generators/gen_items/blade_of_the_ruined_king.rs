use super::*;

impl Generator<ItemData> for BladeOfTheRuinedKing {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let get_scaling = |i| MayFail::Ok(self.passive(0)?.capture_percent(i)? / 100.0);

        let melee_scaling = get_scaling(0)?;
        let ranged_scaling = get_scaling(1)?;

        let damage = |scaling| {
            format!(
                "{EnemyCurrentHealth} - ({numerator} / {scaling})",
                numerator = format_args!(
                    "({scaling} * {EnemyCurrentHealth} * {modifier} - {Ad} + {Ad} * {modifier})",
                    modifier = format_args!("(1 - {scaling} * {PhysicalMultiplier})"),
                ),
            )
        };

        self.damage_type(Physical);
        self.melee_min_dmg(damage(melee_scaling));
        self.ranged_min_dmg(damage(ranged_scaling));
        self.end()
    }
}
