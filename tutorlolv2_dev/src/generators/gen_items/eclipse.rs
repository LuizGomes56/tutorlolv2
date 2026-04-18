use super::*;

impl Generator for Eclipse {
    fn generate(&mut self) -> MayFail {
        let [melee, ranged] = self.capture(Passive(0), |v| v.times(EnemyMaxHealth))?;

        self.melee_min_dmg(melee)
            .ranged_min_dmg(ranged)
            .damage_type(Magic)
            .end()
    }
}
