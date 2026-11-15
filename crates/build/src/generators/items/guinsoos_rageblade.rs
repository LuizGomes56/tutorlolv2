use super::*;

impl Generator for GuinsoosRageblade {
    fn generate(&mut self) -> MayFail {
        // Wrath
        self.damage_type(Magic)
            .trigger(Trigger::EveryHit)
            .min(Named("Wrath"))?
            .end()
    }
}
