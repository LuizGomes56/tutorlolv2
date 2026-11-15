use super::*;

impl Generator for NashorsTooth {
    fn generate(&mut self) -> MayFail {
        self.trigger(Trigger::EveryHit)
            .min(Named("Icathian Bite"))?
            .damage_type(Magic)
            .end()
    }
}
