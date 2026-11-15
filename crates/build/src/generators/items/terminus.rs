use super::*;

impl Generator for Terminus {
    fn generate(&mut self) -> MayFail {
        // Shadow
        self.damage_type(Magic)
            .trigger(Trigger::EveryHit)
            .min(Named("Shadow"))?
            .end()
    }
}
