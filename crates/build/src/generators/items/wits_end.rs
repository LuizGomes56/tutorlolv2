use super::*;

impl Generator for WitsEnd {
    fn generate(&mut self) -> MayFail {
        // Fray
        self.damage_type(Magic)
            .trigger(Trigger::EveryHit)
            .min(Named("Fray"))?
            .end()
    }
}
