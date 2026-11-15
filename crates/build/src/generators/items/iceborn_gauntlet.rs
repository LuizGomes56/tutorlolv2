use super::*;

impl Generator for IcebornGauntlet {
    fn generate(&mut self) -> MayFail {
        // Spellblade
        self.damage_type(Physical)
            .trigger(Trigger::Spellblade)
            .min(Event("Spellblade", Spellblade))?
            .end()
    }
}
