use super::*;

impl Generator for DuskAndDawn {
    fn generate(&mut self) -> MayFail {
        // Spellblade: bonus magic damage on the empowered attack. The other
        // scaling is its healing
        self.damage_type(Magic)
            .attr(OnhitMax)
            .min(Named("Spellblade"))?
            .end()
    }
}
