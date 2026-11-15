use super::*;

impl Generator for LichBane {
    fn generate(&mut self) -> MayFail {
        // Spellblade. The other scaling is the attack speed it grants
        self.damage_type(Magic)
            .trigger(Trigger::Spellblade)
            .min(Named("Spellblade"))?
            .end()
    }
}
