use super::*;

impl Generator for SunderedSky {
    fn generate(&mut self) -> MayFail {
        // Lightshield Strike: bonus damage of the guaranteed critical strike
        self.damage_type(Physical)
            .attr(OnhitMax)
            .set_min(f![0.6 * AttackDamage])
            .end()
    }
}
