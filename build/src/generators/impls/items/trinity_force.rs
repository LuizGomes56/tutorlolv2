use super::*;

impl Generator for TrinityForce {
    fn generate(&mut self) -> MayFail {
        // Spellblade
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min(Passive)?
            .end()
    }
}
