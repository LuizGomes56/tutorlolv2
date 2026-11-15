use super::*;

impl Generator for Bloodsong {
    fn generate(&mut self) -> MayFail {
        // Spellblade
        self.damage_type(Physical)
            .trigger(Trigger::Spellblade)
            .min(Named("Spellblade"))?
            .end()
    }
}
