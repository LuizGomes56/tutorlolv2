use super::*;

impl Generator for RecurveBow {
    fn generate(&mut self) -> MayFail {
        // Sting
        self.damage_type(Physical)
            .trigger(Trigger::EveryHit)
            .min(Named("Sting"))?
            .end()
    }
}
