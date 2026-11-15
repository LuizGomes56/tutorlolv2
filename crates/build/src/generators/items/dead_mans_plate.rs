use super::*;

impl Generator for DeadMansPlate {
    fn generate(&mut self) -> MayFail {
        // Shipwrecker: bonus physical damage on-hit at 100 stacks of Momentum
        self.damage_type(Physical)
            .trigger(Trigger::Charged)
            .maximum_damage(Named("Shipwrecker"))?
            .end()
    }
}
