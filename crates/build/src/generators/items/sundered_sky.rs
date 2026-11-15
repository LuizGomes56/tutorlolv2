use super::*;

impl Generator for SunderedSky {
    fn generate(&mut self) -> MayFail {
        // Lightshield Strike: bonus damage of the guaranteed critical strike
        let bonus = self.critical_bonus(Named("Lightshield Strike"))?;

        self.damage_type(Physical)
            .trigger(Trigger::Charged)
            .set_min(f![bonus / 100 * AttackDamage])
            .end()
    }
}
