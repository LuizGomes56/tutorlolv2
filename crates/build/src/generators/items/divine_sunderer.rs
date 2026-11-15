use super::*;

impl Generator for DivineSunderer {
    fn generate(&mut self) -> MayFail {
        // Spellblade: bonus physical damage on the empowered attack. The other
        // scalings are its healing
        self.damage_type(Physical)
            .trigger(Trigger::Spellblade)
            .min(Event("Spellblade", Spellblade))?
            .end()
    }
}
