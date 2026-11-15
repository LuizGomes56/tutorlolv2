use super::*;

impl Generator for Reverberation {
    fn generate(&mut self) -> MayFail {
        // Resonate
        self.damage_type(Magic)
            .trigger(Trigger::EveryHit)
            .min(Named("Resonate"))?
            .end()
    }
}
