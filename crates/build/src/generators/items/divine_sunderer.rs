use super::*;

impl Generator for DivineSunderer {
    fn generate(&mut self) -> MayFail {
        // Spellblade: bonus physical damage on the empowered attack. The other
        // scalings are its healing
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min(Event("Spellblade", Spellblade))?
            .end()
    }
}
