use super::*;

impl Generator for SunderedSky {
    fn generate(&mut self) -> MayFail {
        // Lightshield Strike: bonus damage of the guaranteed critical strike
        let bonus = self.critical_bonus(Named("Lightshield Strike"))?;

        self.damage_type(Physical)
            .attr(OnhitMax)
            .set_min(f![bonus / 100 * AttackDamage])
            .end()
    }
}
